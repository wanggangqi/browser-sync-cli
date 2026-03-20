<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface ExtensionConfig {
  chrome_extension_id: string | null
  edge_extension_id: string | null
}

const chromeExtensionId = ref('')
const edgeExtensionId = ref('')
const saving = ref(false)
const message = ref('')
const messageType = ref<'success' | 'error'>('success')

onMounted(async () => {
  try {
    const config = await invoke<ExtensionConfig>('get_extension_config')
    chromeExtensionId.value = config.chrome_extension_id || ''
    edgeExtensionId.value = config.edge_extension_id || ''
  } catch (e) {
    console.error('Failed to load config:', e)
  }
})

async function handleSave() {
  saving.value = true
  message.value = ''

  try {
    await invoke('save_extension_config', {
      config: {
        chromeExtensionId: chromeExtensionId.value || null,
        edgeExtensionId: edgeExtensionId.value || null
      }
    })
    messageType.value = 'success'
    message.value = '保存成功！请重启浏览器使配置生效。'
  } catch (e) {
    messageType.value = 'error'
    message.value = `保存失败: ${e}`
  } finally {
    saving.value = false
  }
}

function handleCopyPath() {
  navigator.clipboard.writeText('%LOCALAPPDATA%\\browser-sync-cli')
}
</script>

<template>
  <div class="settings-page">
    <h1 class="page-title">设置</h1>

    <div class="settings-section">
      <h2 class="section-title">扩展 ID 配置</h2>
      <p class="section-desc">
        配置浏览器扩展 ID 以允许 Native Messaging 连接。
        扩展 ID 可在浏览器的扩展管理页面查看。
      </p>

      <div class="form-group">
        <label for="chrome-id">Chrome 扩展 ID</label>
        <input
          id="chrome-id"
          v-model="chromeExtensionId"
          type="text"
          placeholder="例如: abcdefghijklmnopqrstuvwxyz123456"
        />
      </div>

      <div class="form-group">
        <label for="edge-id">Edge 扩展 ID</label>
        <input
          id="edge-id"
          v-model="edgeExtensionId"
          type="text"
          placeholder="例如: abcdefghijklmnopqrstuvwxyz123456"
        />
      </div>

      <div class="form-actions">
        <button class="btn btn-primary" :disabled="saving" @click="handleSave">
          {{ saving ? '保存中...' : '保存配置' }}
        </button>
      </div>

      <div v-if="message" :class="['message', messageType]">
        {{ message }}
      </div>
    </div>

    <div class="settings-section">
      <h2 class="section-title">数据存储位置</h2>
      <div class="path-info">
        <code>%LOCALAPPDATA%\browser-sync-cli</code>
        <button class="btn btn-sm" @click="handleCopyPath">复制路径</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  padding: 24px;
  max-width: 600px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 24px;
  color: #1a1a2e;
}

.settings-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 16px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 8px;
  color: #2c3e50;
}

.section-desc {
  font-size: 14px;
  color: #666;
  margin-bottom: 20px;
  line-height: 1.5;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 6px;
  color: #333;
}

.form-group input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  transition: border-color 0.2s;
}

.form-group input:focus {
  outline: none;
  border-color: #4a90d9;
}

.form-actions {
  margin-top: 20px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #4a90d9;
  color: #fff;
}

.btn-primary:hover:not(:disabled) {
  background: #3a7bc8;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 12px;
  background: #e0e0e0;
  color: #333;
  margin-left: 8px;
}

.btn-sm:hover {
  background: #d0d0d0;
}

.message {
  margin-top: 16px;
  padding: 12px;
  border-radius: 8px;
  font-size: 14px;
}

.message.success {
  background: #d4edda;
  color: #155724;
}

.message.error {
  background: #f8d7da;
  color: #721c24;
}

.path-info {
  display: flex;
  align-items: center;
  background: #f5f5f5;
  padding: 12px;
  border-radius: 8px;
}

.path-info code {
  font-size: 13px;
  color: #333;
}
</style>