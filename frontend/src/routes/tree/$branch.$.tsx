import { FilesSkeleton } from "@/components/files-skeleton/files-skeleton";
import { FilesTable } from "@/components/files-table/files-table";
import { LocationInfo } from "@/components/location-info/location-info";
import { API_URL } from "@/constants";
import type { FilesResponse } from "@/types/files-response";
import { createFileRoute } from "@tanstack/react-router";
import { Suspense } from "react";

const FileViewer = () => {
  const { files } = Route.useLoaderData();
  const { _splat } = Route.useParams();

  return (
    <>
      <LocationInfo route={Route} />
      <Suspense fallback={<FilesSkeleton />}>
        <FilesTable files={files} root={_splat === undefined} />
      </Suspense>
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

    const files = fetch(`${API_URL}/tree/${path}`).then((res) =>
      res.json()
    ) as Promise<FilesResponse>;

    return { files };
  },
});
