const NATIVE_HOST_NAME = 'com.browser_sync.cli';

let nativePort: chrome.runtime.Port | null = null;
let reconnectAttempts = 0;
const MAX_RECONNECT_ATTEMPTS = 5;
const RECONNECT_DELAY = 1000;

export function connectNativeHost(): chrome.runtime.Port {
  if (nativePort) {
    return nativePort;
  }

  nativePort = chrome.runtime.connectNative(NATIVE_HOST_NAME);

  nativePort.onMessage.addListener((message) => {
    console.log('[NativeMessaging] Received:', message);
    if (message.error) {
      console.error('[NativeMessaging] Error from native host:', message.error);
    } else if (message.success) {
      console.log('[NativeMessaging] Operation successful');
    }
  });

  nativePort.onDisconnect.addListener(() => {
    console.log('[NativeMessaging] Disconnected from native host');
    if (chrome.runtime.lastError) {
      console.error('[NativeMessaging] Disconnect error:', chrome.runtime.lastError.message);
    }
    nativePort = null;

    // Attempt reconnection
    if (reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
      reconnectAttempts++;
      console.log(`[NativeMessaging] Attempting reconnect ${reconnectAttempts}/${MAX_RECONNECT_ATTEMPTS}...`);
      setTimeout(() => {
        try {
          connectNativeHost();
        } catch (e) {
          console.error('[NativeMessaging] Reconnect failed:', e);
        }
      }, RECONNECT_DELAY * reconnectAttempts);
    }
  });

  reconnectAttempts = 0;
  return nativePort;
}

export function sendMessage(message: unknown): boolean {
  if (!nativePort) {
    console.warn('[NativeMessaging] No connection, attempting to connect...');
    connectNativeHost();
    if (!nativePort) {
      return false;
    }
  }

  try {
    nativePort.postMessage(message);
    return true;
  } catch (error) {
    console.error('[NativeMessaging] Failed to send message:', error);
    nativePort = null;
    return false;
  }
}

export function disconnectNativeHost(): void {
  if (nativePort) {
    nativePort.disconnect();
    nativePort = null;
  }
}