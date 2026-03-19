export interface BookmarkNode {
  id: string;
  title: string;
  url?: string;
  parentId?: string;
  index?: number;
  dateAdded?: number;
  dateGroupModified?: number;
  children?: BookmarkNode[];
}

export interface BookmarkSyncData {
  version: string;
  lastSync: string;
  bookmarks: BookmarkNode[];
}

export interface NativeMessage {
  type: 'full_sync' | 'incremental_update';
  data: BookmarkSyncData | { action: string; bookmark: BookmarkNode };
}

export type BookmarkEventType = 'created' | 'changed' | 'removed' | 'moved';

export interface BookmarkChange {
  eventType: BookmarkEventType;
  bookmark: BookmarkNode;
  oldParentId?: string;
  oldIndex?: number;
}