import { initBookmarkSync, performFullSync } from '../bookmark-sync.js';

// Initialize immediately when Service Worker loads
initBookmarkSync().catch(console.error);

// Also initialize on these events for redundancy
chrome.runtime.onStartup.addListener(async () => {
  console.log('[Background] Extension starting up...');
  await initBookmarkSync();
});

chrome.runtime.onInstalled.addListener(async (details) => {
  console.log('[Background] Extension installed:', details.reason);
  await initBookmarkSync();
});

// Handle messages from popup (if needed)
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type === 'trigger_sync') {
    performFullSync()
      .then(() => sendResponse({ success: true }))
      .catch((error) => sendResponse({ success: false, error: error.message }));
    return true; // Keep the message channel open for async response
  }

  if (message.type === 'get_status') {
    sendResponse({ status: 'running', connected: true });
    return true;
  }
});

console.log('[Background] Service worker loaded');