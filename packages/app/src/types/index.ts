// 书签节点
export interface BookmarkNode {
  id: string
  title: string
  url?: string
  parentId?: string
  index?: number
  dateAdded?: number
  children?: BookmarkNode[]
}

// 书签同步数据
export interface BookmarkSyncData {
  bookmarks: BookmarkNode[]
}

// 空间类型
export type SpaceType = 'local' | 'remote'

// 浏览器类型
export type BrowserType = 'chrome' | 'edge'

// 空间配置
export interface Space {
  id: string
  name: string
  type: SpaceType
  apiUrl?: string
  apiKey?: string
  browser?: BrowserType
  lastSync?: string
  createdAt: string
  updatedAt: string
}

// 空间配置文件
export interface SpaceConfig {
  version: string
  spaces: Space[]
  activeSpaceId: string
}