<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSpaceStore } from '@/stores/spaceStore'
import Sidebar from '@/components/layout/Sidebar.vue'
import NavigationPage from '@/components/navigation/NavigationPage.vue'
import SpaceManagePage from '@/components/spaces/SpaceManagePage.vue'
import SettingsPage from '@/components/settings/SettingsPage.vue'

const currentPage = ref<'navigation' | 'spaces' | 'settings'>('navigation')

const { spaces, activeSpaceId, loadSpaces } = useSpaceStore()

onMounted(() => {
  loadSpaces()
})

function handleNavigate(page: 'navigation' | 'spaces' | 'settings') {
  currentPage.value = page
}
</script>

<template>
  <div class="app-container">
    <Sidebar
      :current-page="currentPage"
      :spaces="spaces"
      :active-space-id="activeSpaceId"
      @navigate="handleNavigate"
    />
    <main class="main-content">
      <NavigationPage v-if="currentPage === 'navigation'" />
      <SpaceManagePage v-else-if="currentPage === 'spaces'" />
      <SettingsPage v-else-if="currentPage === 'settings'" />
    </main>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
}

.app-container {
  display: flex;
  height: 100%;
}

.main-content {
  flex: 1;
  height: 100%;
  overflow: hidden;
  background: #f5f5f5;
}
</style>
