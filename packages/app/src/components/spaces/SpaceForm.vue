<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Space } from '@/types'

const props = defineProps<{
  space?: Space
  mode: 'create' | 'edit'
}>()

const emit = defineEmits<{
  (e: 'submit', data: { name: string; type: 'local' | 'remote'; apiUrl?: string; apiKey?: string; browser?: 'chrome' | 'edge' }): void
  (e: 'cancel'): void
}>()

const form = ref({
  name: props.space?.name || '',
  type: (props.space?.type || 'local') as 'local' | 'remote',
  apiUrl: props.space?.apiUrl || '',
  apiKey: props.space?.apiKey || '',
  browser: (props.space?.browser || 'chrome') as 'chrome' | 'edge'
})

const isRemote = computed(() => form.value.type === 'remote')
const isEditMode = computed(() => props.mode === 'edit')

// 远程空间 API 返回示例
const apiExample = `{
  "bookmarks": [
    {
      "id": "1",
      "title": "书签文件夹",
      "children": [
        {
          "id": "2",
          "title": "示例书签",
          "url": "https://example.com"
        }
      ]
    }
  ]
}`

function handleSubmit() {
  emit('submit', {
    name: form.value.name,
    type: form.value.type,
    apiUrl: form.value.apiUrl || undefined,
    apiKey: form.value.apiKey || undefined,
    browser: form.value.browser || undefined
  })
}
</script>

<template>
  <form class="space-form" @submit.prevent="handleSubmit">
    <div class="form-group">
      <label for="name">空间名称</label>
      <input
        id="name"
        v-model="form.name"
        type="text"
        required
        placeholder="输入空间名称"
      />
    </div>

    <div class="form-group" v-if="!isEditMode">
      <label for="type">空间类型</label>
      <select id="type" v-model="form.type">
        <option value="local">本地空间</option>
        <option value="remote">远程空间</option>
      </select>
    </div>

    <div class="form-group" v-if="!isEditMode && form.type === 'local'">
      <label for="browser">浏览器类型</label>
      <select id="browser" v-model="form.browser">
        <option value="chrome">Chrome</option>
        <option value="edge">Edge</option>
      </select>
    </div>

    <!-- 编辑模式下显示只读的类型 -->
    <div v-if="isEditMode && props.space" class="form-group">
      <label>空间类型</label>
      <div class="readonly-value">
        <span class="type-badge" :class="props.space.type">
          {{ props.space.type === 'local' ? '本地' : '远程' }}
        </span>
      </div>
    </div>

    <div v-if="isEditMode && props.space && props.space.type === 'local'" class="form-group">
      <label>浏览器类型</label>
      <div class="readonly-value">
        <span class="type-badge" :class="props.space.browser || 'chrome'">
          {{ (props.space.browser || 'chrome') === 'chrome' ? 'Chrome' : 'Edge' }}
        </span>
      </div>
    </div>

    <template v-if="isRemote">
      <div class="form-group">
        <label for="apiUrl">API 地址</label>
        <input
          id="apiUrl"
          v-model="form.apiUrl"
          type="url"
          placeholder="https://api.example.com/bookmarks"
        />
      </div>

      <div class="form-group">
        <label for="apiKey">API 密钥 (可选)</label>
        <input
          id="apiKey"
          v-model="form.apiKey"
          type="password"
          placeholder="输入 API 密钥"
        />
      </div>

      <div v-if="!isEditMode" class="form-group">
        <label>API 返回示例</label>
        <div class="example-box">
          <div class="example-header">
            <span>远程 API 需要返回以下格式的 JSON：</span>
          </div>
          <pre class="example-code">{{ apiExample }}</pre>
        </div>
      </div>
    </template>

    <div class="form-actions">
      <button type="button" class="btn btn-secondary" @click="$emit('cancel')">
        取消
      </button>
      <button type="submit" class="btn btn-primary">
        {{ mode === 'create' ? '创建' : '保存' }}
      </button>
    </div>
  </form>
</template>

<style scoped>
.space-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group label {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.form-group input,
.form-group select {
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: #4a90d9;
}

.readonly-value {
  padding: 10px 12px;
  background: #f5f5f5;
  border-radius: 6px;
  font-size: 14px;
}

.type-badge {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
}

.type-badge.local {
  background: #e8f5e9;
  color: #2e7d32;
}

.type-badge.remote {
  background: #e3f2fd;
  color: #1565c0;
}

.example-box {
  background: #f8f9fa;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  overflow: hidden;
}

.example-header {
  padding: 8px 12px;
  background: #e9ecef;
  font-size: 13px;
  color: #495057;
  border-bottom: 1px solid #e0e0e0;
}

.example-code {
  margin: 0;
  padding: 12px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.5;
  color: #333;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 300px;
  overflow-y: auto;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 8px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #4a90d9;
  color: #fff;
}

.btn-primary:hover {
  background: #3a7fc8;
}

.btn-secondary {
  background: #f0f0f0;
  color: #333;
}

.btn-secondary:hover {
  background: #e0e0e0;
}
</style>
