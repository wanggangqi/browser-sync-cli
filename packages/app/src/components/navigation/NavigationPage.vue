<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { useSpaceStore } from '@/stores/spaceStore'
import BookmarkTree from './BookmarkTree.vue'
import type { BookmarkNode, BookmarkSyncData } from '@/types'

const { spaces, getSpaceCache } = useSpaceStore()

const bookmarks = ref<BookmarkNode[]>([])
const searchQuery = ref('')
const searchSpaceId = ref('all') // 'all' 表示全部空间
const isLoading = ref(false)
const treeRef = ref<InstanceType<typeof BookmarkTree> | null>(null)

// 当前空间的数据（用于"全部"模式）
const spaceDataMap = ref<Map<string, BookmarkNode[]>>(new Map())

// 过滤掉空名称的根节点，直接取其 children
function flattenBookmarks(nodes: BookmarkNode[]): BookmarkNode[] {
  // 如果只有一个根节点且没有名称，直接返回其 children
  if (nodes.length === 1 && nodes[0].title === '' && nodes[0].children) {
    return nodes[0].children
  }
  return nodes
}

// 为节点 ID 添加前缀，避免不同空间之间的 ID 冲突
function prefixNodeIds(nodes: BookmarkNode[], prefix: string): BookmarkNode[] {
  return nodes.map(node => ({
    ...node,
    id: `${prefix}-${node.id}`,
    parentId: node.parentId ? `${prefix}-${node.parentId}` : undefined,
    children: node.children ? prefixNodeIds(node.children, prefix) : undefined
  }))
}

// 加载指定空间的书签
async function loadSpaceBookmarks(spaceId: string): Promise<BookmarkNode[]> {
  try {
    // 查找空间配置以获取浏览器类型
    const space = spaces.value.find(s => s.id === spaceId)

    // 如果空间有 browser 属性，读取对应浏览器的书签
    if (space?.browser) {
      const data = await invoke<BookmarkSyncData>('get_bookmarks', { browser: space.browser })
      return flattenBookmarks(data.bookmarks || [])
    }

    // 其他空间（remote 类型）尝试获取缓存
    const cache = await getSpaceCache(spaceId)
    if (cache) {
      return flattenBookmarks(cache.bookmarks || [])
    }

    return []
  } catch (error) {
    console.error('Failed to load bookmarks for space:', spaceId, error)
    return []
  }
}

// 加载所有空间的书签（合并模式）
async function loadAllSpacesBookmarks() {
  isLoading.value = true
  spaceDataMap.value = new Map()

  try {
    for (const space of spaces.value) {
      const spaceBookmarks = await loadSpaceBookmarks(space.id)
      // 为每个空间的书签 ID 添加前缀，避免冲突
      const prefixedBookmarks = prefixNodeIds(spaceBookmarks, space.id)
      spaceDataMap.value.set(space.id, prefixedBookmarks)
    }

    // 合并所有空间的书签，每个空间作为根节点
    const merged: BookmarkNode[] = spaces.value.map(space => ({
      id: `space-root-${space.id}`,
      title: space.name,
      url: undefined,
      children: spaceDataMap.value.get(space.id) || []
    }))

    bookmarks.value = merged
  } catch (error) {
    console.error('Failed to load all spaces bookmarks:', error)
    bookmarks.value = []
  } finally {
    isLoading.value = false
  }
}

// 加载单个空间的书签
async function loadSingleSpaceBookmarks(spaceId: string) {
  isLoading.value = true
  try {
    bookmarks.value = await loadSpaceBookmarks(spaceId)
  } catch (error) {
    console.error('Failed to load bookmarks:', error)
    bookmarks.value = []
  } finally {
    isLoading.value = false
  }
}

async function loadBookmarks() {
  if (searchSpaceId.value === 'all') {
    await loadAllSpacesBookmarks()
  } else {
    await loadSingleSpaceBookmarks(searchSpaceId.value)
  }
}

// 监听空间切换
watch(searchSpaceId, () => {
  loadBookmarks()
})

// 监听 spaces 变化（当空间列表更新时）
watch(spaces, () => {
  if (searchSpaceId.value === 'all') {
    loadAllSpacesBookmarks()
  }
}, { deep: true })

let unlisten: UnlistenFn | null = null

onMounted(async () => {
  await loadBookmarks()
  unlisten = await listen('bookmark-changed', () => {
    loadBookmarks()
  })
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})

async function openUrl(url: string) {
  try {
    await invoke('open_url', { url })
  } catch (error) {
    console.error('Failed to open URL:', error)
  }
}

function expandAll() {
  treeRef.value?.expandAll()
}

function collapseAll() {
  treeRef.value?.collapseAll()
}
</script>

<template>
  <div class="navigation-page">
    <div class="toolbar">
      <div class="search-wrapper">
        <select v-model="searchSpaceId" class="space-select">
          <option value="all">全部</option>
          <option v-for="space in spaces" :key="space.id" :value="space.id">
            {{ space.name }}
          </option>
        </select>
        <input
          v-model="searchQuery"
          type="text"
          class="search-input"
          placeholder="搜索书签..."
        />
      </div>
      <div class="actions">
        <button class="btn btn-sm btn-primary" @click="loadBookmarks" :disabled="isLoading">
          {{ isLoading ? '加载中...' : '刷新' }}
        </button>
        <button class="btn btn-sm btn-secondary" @click="expandAll" title="展开全部">
          展开
        </button>
        <button class="btn btn-sm btn-secondary" @click="collapseAll" title="折叠全部">
          折叠
        </button>
      </div>
    </div>

    <div v-if="isLoading" class="loading">
      <div class="spinner"></div>
    </div>

    <div v-else class="bookmark-tree-wrapper">
      <BookmarkTree
        ref="treeRef"
        :bookmarks="bookmarks"
        :search-query="searchQuery"
        @open-url="openUrl"
      />
    </div>
  </div>
</template>

<style scoped>
.navigation-page {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.toolbar {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-bottom: 1px solid #e0e0e0;
  background: #fafafa;
  align-items: center;
}

.search-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: #fff;
  overflow: hidden;
}

.search-wrapper:focus-within {
  border-color: #4a90d9;
}

.space-select {
  padding: 8px 12px;
  border: none;
  border-right: 1px solid #e0e0e0;
  font-size: 14px;
  background: #f9f9f9;
  cursor: pointer;
  outline: none;
  max-width: 150px;
}

.space-select:focus {
  background: #f0f0f0;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  border: none;
  font-size: 14px;
  outline: none;
}

.actions {
  display: flex;
  gap: 6px;
}

.btn {
  padding: 8px 14px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.btn-primary {
  background: #4a90d9;
  color: #fff;
}

.btn-primary:hover:not(:disabled) {
  background: #3a7fc8;
}

.btn-secondary {
  background: #f0f0f0;
  color: #333;
}

.btn-secondary:hover {
  background: #e0e0e0;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-sm {
  padding: 7px 12px;
  font-size: 13px;
}

.loading {
  display: flex;
  justify-content: center;
  align-items: center;
  flex: 1;
}

.bookmark-tree-wrapper {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid #f0f0f0;
  border-top-color: #4a90d9;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
