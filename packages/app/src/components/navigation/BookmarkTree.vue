<script setup lang="ts">
import { ref, watch } from 'vue'
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

// 搜索时自动展开包含匹配项的父节点
watch(() => props.searchQuery, (query) => {
  if (query) {
    const matchingParentIds = new Set<string>()
    findMatchingParents(props.bookmarks, query.toLowerCase(), matchingParentIds, new Set<string>())
    expandedIds.value = matchingParentIds
  }
}, { immediate: true })

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
    <div v-if="bookmarks.length === 0" class="empty-state">
      <p v-if="searchQuery">没有找到匹配的书签</p>
      <p v-else>暂无书签</p>
    </div>
    <BookmarkTreeNode
      v-for="node in bookmarks"
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