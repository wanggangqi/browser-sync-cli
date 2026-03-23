<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import BookmarkTreeNode from './BookmarkTreeNode.vue'
import type { BookmarkNode } from '@/types'

const props = defineProps<{
  bookmarks: BookmarkNode[]
  searchQuery?: string
}>()

const emit = defineEmits<{
  (e: 'openUrl', url: string): void
}>()

// 展开的节点 ID 集合
const expandedIds = ref<Set<string>>(new Set())

// 过滤后的树形数据（搜索时使用）
const filteredBookmarks = computed(() => {
  if (!props.searchQuery) {
    return props.bookmarks
  }
  const query = props.searchQuery.toLowerCase()
  return filterTree(props.bookmarks, query)
})

// 递归过滤树节点，只保留匹配的节点及其父节点
function filterTree(nodes: BookmarkNode[], query: string): BookmarkNode[] {
  const result: BookmarkNode[] = []

  for (const node of nodes) {
    const selfMatch = node.title.toLowerCase().includes(query) ||
                      (node.url && node.url.toLowerCase().includes(query))

    let filteredChildren: BookmarkNode[] = []
    if (node.children?.length) {
      filteredChildren = filterTree(node.children, query)
    }

    // 如果自己匹配，或者子节点中有匹配的，就保留这个节点
    if (selfMatch || filteredChildren.length > 0) {
      result.push({
        ...node,
        children: filteredChildren.length > 0 ? filteredChildren : node.children
      })
    }
  }

  return result
}

// 查找所有包含匹配项的父节点
function findMatchingParents(
  nodes: BookmarkNode[],
  query: string,
  matchingParents: Set<string>,
  currentPath: Set<string>
): boolean {
  let hasMatch = false
  for (const node of nodes) {
    const titleMatch = node.title.toLowerCase().includes(query)
    const urlMatch = node.url?.toLowerCase().includes(query) ?? false
    const selfMatch = titleMatch || urlMatch

    currentPath.add(node.id)

    let childHasMatch = false
    if (node.children?.length) {
      childHasMatch = findMatchingParents(node.children, query, matchingParents, currentPath)
    }

    if (selfMatch || childHasMatch) {
      hasMatch = true
      // 将所有父节点添加到展开集合
      for (const parentId of currentPath) {
        if (parentId !== node.id) {
          matchingParents.add(parentId)
        }
      }
      // 如果自己有子节点且子节点有匹配，展开自己
      if (childHasMatch && node.children?.length) {
        matchingParents.add(node.id)
      }
    }

    currentPath.delete(node.id)
  }
  return hasMatch
}

// 搜索时自动展开包含匹配项的父节点
watch(() => props.searchQuery, (query) => {
  if (query && query.trim()) {
    const matchingParentIds = new Set<string>()
    findMatchingParents(props.bookmarks, query.toLowerCase(), matchingParentIds, new Set<string>())
    expandedIds.value = matchingParentIds
  } else {
    // 清空搜索时重置展开状态
    expandedIds.value = new Set()
  }
}, { immediate: true })

function toggleNode(id: string) {
  const newSet = new Set(expandedIds.value)
  if (newSet.has(id)) {
    newSet.delete(id)
  } else {
    newSet.add(id)
  }
  expandedIds.value = newSet
}

function handleNodeClick(node: BookmarkNode) {
  if (node.url) {
    emit('openUrl', node.url)
  }
}

// 展开/折叠全部
function expandAll() {
  const allFolderIds = new Set<string>()
  function collectFolders(nodes: BookmarkNode[]) {
    for (const node of nodes) {
      if (node.children?.length) {
        allFolderIds.add(node.id)
        collectFolders(node.children)
      }
    }
  }
  collectFolders(props.bookmarks)
  expandedIds.value = allFolderIds
}

function collapseAll() {
  expandedIds.value = new Set()
}

defineExpose({
  expandAll,
  collapseAll
})
</script>

<template>
  <div class="bookmark-tree">
    <div v-if="filteredBookmarks.length === 0" class="empty-state">
      <p v-if="searchQuery">没有找到匹配的书签</p>
      <p v-else>暂无书签</p>
    </div>
    <BookmarkTreeNode
      v-for="node in filteredBookmarks"
      :key="node.id"
      :node="node"
      :search-query="searchQuery"
      :expanded-ids="expandedIds"
      @toggle="toggleNode"
      @click="handleNodeClick"
    />
  </div>
</template>

<style scoped>
.bookmark-tree {
  padding: 8px;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #666;
}
</style>