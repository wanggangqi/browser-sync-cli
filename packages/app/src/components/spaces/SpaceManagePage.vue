<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSpaceStore } from '@/stores/spaceStore'
import SpaceForm from './SpaceForm.vue'
import type { Space } from '@/types'

const {
  spaces,
  isLoading,
  error,
  loadSpaces,
  createSpace,
  updateSpace,
  deleteSpace,
  fetchRemoteBookmarks
} = useSpaceStore()

const syncingSpaceId = ref<string | null>(null)

const showModal = ref(false)
const modalMode = ref<'create' | 'edit'>('create')
const editingSpace = ref<Space | undefined>()

onMounted(() => {
  loadSpaces()
})

function openCreateModal() {
  modalMode.value = 'create'
  editingSpace.value = undefined
  showModal.value = true
}

function openEditModal(space: Space) {
  modalMode.value = 'edit'
  editingSpace.value = space
  showModal.value = true
}

function closeModal() {
  showModal.value = false
  editingSpace.value = undefined
}

async function handleSubmit(data: { name: string; type: 'local' | 'remote'; apiUrl?: string; apiKey?: string }) {
  try {
    if (modalMode.value === 'create') {
      await createSpace(data.name, data.type, data.apiUrl, data.apiKey)
    } else if (editingSpace.value) {
      await updateSpace({
        ...editingSpace.value,
        name: data.name,
        type: data.type,
        apiUrl: data.apiUrl,
        apiKey: data.apiKey
      })
    }
    closeModal()
  } catch (e) {
    console.error('Failed to save space:', e)
  }
}

// 默认空间不可编辑和删除
function isDefaultSpace(space: Space) {
  return space.id === 'default-chrome' || space.id === 'default-edge'
}

async function handleDelete(space: Space) {
  if (isDefaultSpace(space)) {
    alert('无法删除默认空间')
    return
  }
  if (confirm(`确定要删除空间 "${space.name}" 吗？`)) {
    try {
      await deleteSpace(space.id)
    } catch (e) {
      alert(String(e))
    }
  }
}

async function handleSync(space: Space) {
  if (space.type !== 'remote' || !space.apiUrl) {
    return
  }
  syncingSpaceId.value = space.id
  try {
    await fetchRemoteBookmarks(space.id, space.apiUrl, space.apiKey)
  } catch (e) {
    alert(`同步失败：${e}`)
  } finally {
    syncingSpaceId.value = null
  }
}

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleString('zh-CN')
}
</script>

<template>
  <div class="space-manage-page">
    <div class="page-header">
      <h1>空间管理</h1>
      <button class="btn btn-primary" @click="openCreateModal">
        + 新建空间
      </button>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-if="isLoading" class="loading">
      <div class="spinner"></div>
    </div>

    <div v-else class="space-list">
      <div
        v-for="space in spaces"
        :key="space.id"
        class="space-card"
      >
        <div class="space-header">
          <div class="space-info">
            <h3 class="space-name">{{ space.name }}</h3>
            <span class="space-type" :class="space.type">
              {{ space.type === 'local' ? '本地' : '远程' }}
            </span>
            <span v-if="space.type === 'local'" class="space-browser" :class="space.browser || 'chrome'">
              {{ (space.browser || 'chrome') === 'chrome' ? 'Chrome' : 'Edge' }}
            </span>
          </div>
          <div class="space-actions">
            <button
              v-if="space.type === 'remote' && space.apiUrl"
              class="btn btn-sm btn-primary"
              @click="handleSync(space)"
              :disabled="syncingSpaceId === space.id"
            >
              {{ syncingSpaceId === space.id ? '同步中...' : '同步' }}
            </button>
            <button
              v-if="!isDefaultSpace(space)"
              class="btn btn-sm btn-secondary"
              @click="openEditModal(space)"
            >
              编辑
            </button>
            <button
              v-if="!isDefaultSpace(space)"
              class="btn btn-sm btn-danger"
              @click="handleDelete(space)"
            >
              删除
            </button>
          </div>
        </div>

        <div class="space-details">
          <div v-if="space.type === 'remote' && space.apiUrl" class="detail-item">
            <span class="detail-label">API 地址:</span>
            <span class="detail-value">{{ space.apiUrl }}</span>
          </div>
          <div v-if="space.lastSync" class="detail-item">
            <span class="detail-label">最后同步:</span>
            <span class="detail-value">{{ formatDate(space.lastSync) }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">创建时间:</span>
            <span class="detail-value">{{ formatDate(space.createdAt) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal -->
    <Teleport to="body">
      <div v-if="showModal" class="modal-overlay" @click.self="closeModal">
        <div class="modal-content">
          <div class="modal-header">
            <h2>{{ modalMode === 'create' ? '新建空间' : '编辑空间' }}</h2>
            <button class="modal-close" @click="closeModal">&times;</button>
          </div>
          <div class="modal-body">
            <SpaceForm
              :space="editingSpace"
              :mode="modalMode"
              @submit="handleSubmit"
              @cancel="closeModal"
            />
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.space-manage-page {
  padding: 24px;
  height: 100%;
  overflow-y: auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.page-header h1 {
  margin: 0;
  font-size: 24px;
  color: #333;
}

.error-message {
  padding: 12px;
  background: #fee;
  border: 1px solid #fcc;
  border-radius: 6px;
  color: #c00;
  margin-bottom: 16px;
}

.loading {
  display: flex;
  justify-content: center;
  padding: 40px;
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
  to { transform: rotate(360deg); }
}

.space-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.space-card {
  background: #fff;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s;
}

.space-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.space-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.space-name {
  margin: 0;
  font-size: 18px;
  color: #333;
}

.space-type {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.space-type.local {
  background: #e8f5e9;
  color: #2e7d32;
}

.space-type.remote {
  background: #e3f2fd;
  color: #1565c0;
}

.space-browser {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.space-browser.chrome {
  background: #e8f5e9;
  color: #2e7d32;
}

.space-browser.edge {
  background: #e3f2fd;
  color: #1565c0;
}

.space-actions {
  display: flex;
  gap: 8px;
}

.space-details {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  font-size: 14px;
  color: #666;
}

.detail-item {
  display: flex;
  gap: 6px;
}

.detail-label {
  color: #999;
}

.btn {
  padding: 8px 16px;
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

.btn-danger {
  background: #fee;
  color: #c00;
}

.btn-danger:hover {
  background: #fdd;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 13px;
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: #fff;
  border-radius: 12px;
  width: 90%;
  max-width: 480px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
}

.modal-close {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  font-size: 24px;
  cursor: pointer;
  color: #999;
}

.modal-close:hover {
  color: #333;
}

.modal-body {
  padding: 20px;
}
</style>