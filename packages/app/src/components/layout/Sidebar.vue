<script setup lang="ts">
import type { Space } from '@/types'

defineProps<{
  currentPage: 'navigation' | 'spaces' | 'settings'
  spaces: Space[]
  activeSpaceId: string
}>()

defineEmits<{
  (e: 'navigate', page: 'navigation' | 'spaces' | 'settings'): void
}>()

const menuItems = [
  { id: 'navigation' as const, icon: '📑', label: '导航' },
  { id: 'spaces' as const, icon: '🗂️', label: '空间管理' },
  { id: 'settings' as const, icon: '⚙️', label: '设置' }
]
</script>

<template>
  <aside class="sidebar">
    <!-- Logo -->
    <div class="sidebar-header">
      <div class="logo">
        <svg viewBox="0 0 512 512" class="logo-icon">
          <defs>
            <linearGradient id="sidebar-bg-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#1a3a5c"/>
              <stop offset="100%" stop-color="#2c5282"/>
            </linearGradient>
            <linearGradient id="sidebar-cross-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stop-color="#ffd700"/>
              <stop offset="50%" stop-color="#ffb700"/>
              <stop offset="100%" stop-color="#ff8c00"/>
            </linearGradient>
          </defs>
          <rect width="512" height="512" rx="100" fill="url(#sidebar-bg-gradient)"/>
          <rect x="236" y="100" width="40" height="312" rx="8" fill="url(#sidebar-cross-gradient)"/>
          <rect x="100" y="236" width="312" height="40" rx="8" fill="url(#sidebar-cross-gradient)"/>
        </svg>
      </div>
    </div>

    <!-- 导航菜单 -->
    <nav class="nav-menu">
      <button
        v-for="item in menuItems"
        :key="item.id"
        class="nav-item"
        :class="{ active: currentPage === item.id }"
        @click="$emit('navigate', item.id)"
      >
        <span class="nav-icon-wrapper">
          <span class="nav-icon">{{ item.icon }}</span>
        </span>
        <span class="nav-label">{{ item.label }}</span>
      </button>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 80px;
  height: 100%;
  background: #2c3e50;
  color: #fff;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-header {
  padding: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 70px;
}

.logo {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  overflow: hidden;
}

.logo-icon {
  width: 100%;
  height: 100%;
}

.nav-menu {
  flex: 1;
  padding: 12px 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-item {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 8px;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.7);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.nav-item.active {
  color: #fff;
}

.nav-item.active .nav-icon-wrapper {
  background: rgba(74, 144, 217, 0.4);
}

/* 图标容器 - 保持正方形 */
.nav-icon-wrapper {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.1);
  transition: all 0.2s;
}

.nav-item:hover .nav-icon-wrapper {
  background: rgba(255, 255, 255, 0.15);
}

.nav-icon {
  font-size: 20px;
  flex-shrink: 0;
}

.nav-label {
  font-size: 12px;
  line-height: 1.2;
}
</style>
