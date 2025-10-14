export type FilesResponse = {
  blobs: FileInfo[];
  trees: FileInfo[];
};

export type FileInfo = {
  name: string;
  last_commit_message: string;
  last_commit_timestamp: string;
  last_commit_id: string;
};
