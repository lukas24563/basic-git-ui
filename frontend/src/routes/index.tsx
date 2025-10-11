import { API_URL } from "@/constants";
import type { FilesResponse } from "@/types/files-response";
import type { RepositoryInfo } from "@/types/repository-info";
import { createFileRoute } from "@tanstack/react-router";
import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";
import { Dropdown } from "primereact/dropdown";

type FilesEntry = {
  name: string;
  type: "blob" | "tree";
};

const nameBodyEntry = (entry: FilesEntry) => {
  return entry.type === "tree" ? "📁 " + entry.name : "📄 " + entry.name;
};

const App = () => {
  const data = Route.useLoaderData();

  const rows: FilesEntry[] = [];
  data.files.trees.forEach((tree: string) => {
    rows.push({ name: tree, type: "tree" });
  });
  data.files.blobs.forEach((blob: string) => {
    rows.push({ name: blob, type: "blob" });
  });

  return (
    <div>
      <h1>{data.info.name}</h1>
      <Dropdown value={data.info.main_branch} options={data.branches} />
      <DataTable value={rows}>
        <Column body={nameBodyEntry} header="Name" />
      </DataTable>
    </div>
  );
};

export const Route = createFileRoute("/")({
  component: App,
  loader: async () => {
    const info = fetch(API_URL + "/info").then((res) => res.json());
    const branches = fetch(API_URL + "/branches").then((res) => res.json());
    const files = fetch(API_URL + "/files").then((res) => res.json());
    return {
      info: (await info) as RepositoryInfo,
      branches: await branches,
      files: (await files) as FilesResponse,
    };
  },
});
