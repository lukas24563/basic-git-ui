import { LocationInfo } from "@/components/location-info/location-info";
import { API_URL } from "@/constants";
import type { FileInfo, FilesResponse } from "@/types/files-response";
import { formatRelativeTime } from "@/utils/format-relative-time";
import {
  createFileRoute,
  Link,
  type ResolveParams,
} from "@tanstack/react-router";
import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";
import Styles from "./$branch.$.module.css";

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

const FileViewer = () => {
  const files = Route.useLoaderData();
  const { _splat } = Route.useParams();

  const rows: FilesEntry[] = [];

  if (_splat) {
    rows.push({
      name: "..",
      type: "tree",
      lastCommitMessage: "",
      lastUpdate: "",
    });
  }

  files.trees.forEach((tree: FileInfo) => {
    rows.push({
      name: tree.name,
      type: "tree",
      lastCommitMessage: tree.last_commit_message,
      lastUpdate: formatRelativeTime(tree.last_commit_timestamp),
    });
  });
  files.blobs.forEach((blob: FileInfo) => {
    rows.push({
      name: blob.name,
      type: "blob",
      lastCommitMessage: blob.last_commit_message,
      lastUpdate: formatRelativeTime(blob.last_commit_timestamp),
    });
  });

  return (
    <>
      <LocationInfo route={Route} />
      <DataTable value={rows} rowClassName={() => Styles.row}>
        <Column
          className={Styles.nameColumn}
          body={nameBodyEntry}
          header="Name"
        />
        <Column field="lastCommitMessage" header="Last commit" />
        <Column field="lastUpdate" header="Last update" />
      </DataTable>
    </>
  );
};

export const Route = createFileRoute("/tree/$branch/$")({
  component: FileViewer,
  loader: async ({ params }) => {
    const branch_path = encodeURIComponent(params.branch);
    const path = !params._splat
      ? branch_path
      : `${branch_path}/${params._splat}`;

    const files = (await fetch(`${API_URL}/tree/${path}`).then((res) =>
      res.json()
    )) as FilesResponse;

    return files;
  },
});
