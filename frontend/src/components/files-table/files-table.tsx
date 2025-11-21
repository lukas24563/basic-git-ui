import type { FileInfo, FilesResponse } from "@/types/files-response";
import { formatRelativeTime } from "@/utils/format-relative-time";
import { Link, type ResolveParams } from "@tanstack/react-router";
import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";
import { use, type FunctionComponent, type PropsWithChildren } from "react";
import Styles from "./files-table.module.css";

type FileTableProps = {
  files: Promise<FilesResponse>;
  root: boolean;
};

type FilesEntry = {
  name: string;
  type: "blob" | "tree";
  lastCommitMessage: string;
  lastUpdate: string;
};

const nameBodyEntry = (entry: FilesEntry) => {
  const icon = entry.type === "tree" ? "fa-folder" : "fa-file";
  const to = entry.type === "tree" ? "." : "/blob/$branch/$";

  const getParams = (current: Partial<ResolveParams<"/tree/$branch/$">>) => {
    const splat =
      entry.name === ".."
        ? current._splat?.substring(0, current._splat.lastIndexOf("/"))
        : `${current._splat}/${entry.name}`;

    return { ...current, _splat: splat };
  };

  return (
    <Link to={to} params={getParams} className={Styles.fileNameLink}>
      <i className={`${Styles.icon} fa-regular ${icon}`} />
      <span>{entry.name}</span>
    </Link>
  );
};

export const FilesTable: FunctionComponent<
  PropsWithChildren<FileTableProps>
> = (props) => {
  const files = use(props.files);

  const rows: FilesEntry[] = [];

  if (props.root) {
    rows.push({
      name: "..",
      type: "tree",
      lastCommitMessage: "",
      lastUpdate: "",
    });
  }

  files.trees.forEach((tree: FileInfo) =>
    rows.push({
      name: tree.name,
      type: "tree",
      lastCommitMessage: tree.last_commit_message,
      lastUpdate: formatRelativeTime(tree.last_commit_timestamp),
    })
  );

  files.blobs.forEach((blob: FileInfo) =>
    rows.push({
      name: blob.name,
      type: "blob",
      lastCommitMessage: blob.last_commit_message,
      lastUpdate: formatRelativeTime(blob.last_commit_timestamp),
    })
  );

  return (
    <DataTable value={rows} rowClassName={() => Styles.row}>
      <Column
        className={Styles.nameColumn}
        body={nameBodyEntry}
        header="Name"
      />
      <Column field="lastCommitMessage" header="Last commit" />
      <Column field="lastUpdate" header="Last update" />
    </DataTable>
  );
};
