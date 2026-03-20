import { sendMessage, connectNativeHost } from './native-messaging.js';

interface ChromeBookmarkTreeNode {
  id: string;
  title: string;
  url?: string;
  parentId?: string;
  index?: number;
  dateAdded?: number;
  dateGroupModified?: number;
  children?: ChromeBookmarkTreeNode[];
}

interface BookmarkSyncData {
  bookmarks: ChromeBookmarkTreeNode[];
}

// 检测当前浏览器类型
function getBrowserType(): 'chrome' | 'edge' {
  if (typeof navigator !== 'undefined') {
    const userAgent = navigator.userAgent;
    if (userAgent.includes('Edg/')) {
      return 'edge';
    }
  }
  // 默认返回 chrome
  return 'chrome';
}

function convertBookmarkTree(nodes: ChromeBookmarkTreeNode[]): ChromeBookmarkTreeNode[] {
  return nodes.map(node => ({
    id: node.id,
    title: node.title,
    url: node.url,
    parentId: node.parentId,
    index: node.index,
    dateAdded: node.dateAdded,
    dateGroupModified: node.dateGroupModified,
    children: node.children ? convertBookmarkTree(node.children) : undefined
  }));
}

export async function performFullSync(): Promise<void> {
  try {
    const tree = await chrome.bookmarks.getTree();
    const syncData: BookmarkSyncData = {
      bookmarks: convertBookmarkTree(tree)
    };

    const success = sendMessage({
      type: 'full_sync',
      data: syncData,
      browser: getBrowserType()
    });

    if (success) {
      console.log('[BookmarkSync] Full sync completed');
    } else {
      console.error('[BookmarkSync] Full sync failed - cannot connect to native host');
    }
  } catch (error) {
    console.error('[BookmarkSync] Full sync error:', error);
  }
}

export async function initBookmarkSync(): Promise<void> {
  // Connect to native host
  connectNativeHost();

  // Perform initial full sync
  await performFullSync();

  // Listen for bookmark changes
  chrome.bookmarks.onCreated.addListener(async (id, bookmark) => {
    console.log('[BookmarkSync] Bookmark created:', id);
    await performFullSync();
  });

  chrome.bookmarks.onRemoved.addListener(async (id, removeInfo) => {
    console.log('[BookmarkSync] Bookmark removed:', id);
    await performFullSync();
  });

  chrome.bookmarks.onChanged.addListener(async (id, changeInfo) => {
    console.log('[BookmarkSync] Bookmark changed:', id);
    await performFullSync();
  });

  chrome.bookmarks.onMoved.addListener(async (id, moveInfo) => {
    console.log('[BookmarkSync] Bookmark moved:', id);
    await performFullSync();
  });

  chrome.bookmarks.onChildrenReordered.addListener(async (id, reorderInfo) => {
    console.log('[BookmarkSync] Bookmarks reordered:', id);
    await performFullSync();
  });

  console.log('[BookmarkSync] Bookmark sync initialized');
}