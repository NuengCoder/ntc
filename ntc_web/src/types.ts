export interface FileItem {
  id: number
  name: string
  isDir: boolean
  size: number
}

export interface FilesResponse {
  path: string
  items: FileItem[]
  error?: string
}
