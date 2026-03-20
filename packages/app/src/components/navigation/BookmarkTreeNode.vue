<script setup lang="ts">
import { computed } from 'vue'
import type { BookmarkNode } from '@/types'

const props = defineProps<{
  node: BookmarkNode
  searchQuery?: string
  expandedIds: Set<string>
}>()

const emit = defineEmits<{
  (e: 'toggle', id: string): void
  (e: 'click', node: BookmarkNode): void
}>()

const isFolder = computed(() => !props.node.url && props.node.children?.length)
const isExpanded = computed(() => props.expandedIds.has(props.node.id))

// 检查标题是否匹配搜索
const titleMatch = computed(() => {
  if (!props.searchQuery) return false
  return props.node.title.toLowerCase().includes(props.searchQuery.toLowerCase())
})

// 检查 URL 是否匹配搜索
const urlMatch = computed(() => {
  if (!props.searchQuery || !props.node.url) return false
  return props.node.url.toLowerCase().includes(props.searchQuery.toLowerCase())
})

// 高亮文本
function highlightText(text: string, query: string): string {
  if (!query) return text
  const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return text.replace(regex, '<mark>$1</mark>')
}

const highlightedTitle = computed(() => {
  return highlightText(props.node.title, props.searchQuery || '')
})

const highlightedUrl = computed(() => {
  if (!props.node.url) return ''
  return highlightText(props.node.url, props.searchQuery || '')
})

function toggle() {
  emit('toggle', props.node.id)
}

function handleClick() {
  if (props.node.url) {
    emit('click', props.node)
  } else if (isFolder.value) {
    toggle()
  }
}

function handleIconError(event: Event) {
  const target = event.target as HTMLImageElement
  target.style.display = 'none'
}
</script>

<template>
  <div class="tree-node">
    <div
      class="node-content"
      :class="{
        'is-folder': isFolder,
        'is-expanded': isExpanded,
        'is-match': titleMatch || urlMatch
      }"
      @click="handleClick"
    >
      <!-- 展开箭头 -->
      <span
        v-if="isFolder"
        class="expand-icon"
        @click.stop="toggle"
      >
        {{ isExpanded ? '▼' : '▶' }}
      </span>
      <span v-else class="expand-icon-placeholder"></span>

      <!-- 图标 -->
      <img
        v-if="node.url"
        :src="`https://www.google.com/s2/favicons?domain=${node.url}&sz=16`"
        class="node-icon"
        alt=""
        @error="handleIconError"
      />
      <span v-else class="folder-icon">📁</span>

      <!-- 标题 -->
      <span class="node-title" v-html="highlightedTitle"></span>

      <!-- URL -->
      <span v-if="node.url" class="node-url" v-html="highlightedUrl"></span>
    </div>

    <!-- 子节点 -->
    <div v-if="isFolder && isExpanded" class="children">
      <BookmarkTreeNode
        v-for="child in node.children"
        :key="child.id"
        :node="child"
        :search-query="searchQuery"
        :expanded-ids="expandedIds"
        @toggle="$emit('toggle', $event)"
        @click="$emit('click', $event)"
      />
    </div>
  </div>
</template>

<style scoped>
.tree-node {
  user-select: none;
}

.node-content {
  display: flex;
  align-items: center;
  padding: 6px 8px;
  cursor: pointer;
  border-radius: 4px;
  gap: 8px;
}

.node-content:hover {
  background-color: #f0f0f0;
}

.node-content.is-match {
  background-color: #fff3cd;
}

.expand-icon {
  width: 16px;
  font-size: 10px;
  color: #666;
  cursor: pointer;
  flex-shrink: 0;
}

.expand-icon-placeholder {
  width: 16px;
  flex-shrink: 0;
}

.node-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.folder-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.node-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.node-title :deep(mark) {
  background-color: #ffc107;
  padding: 0 2px;
  border-radius: 2px;
}

.node-url {
  font-size: 12px;
  color: #666;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.node-url :deep(mark) {
  background-color: #ffc107;
  padding: 0 2px;
  border-radius: 2px;
}

.children {
  padding-left: 20px;
}
</style>