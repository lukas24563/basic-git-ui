import { LocationInfo } from "@/components/location-info/location-info";
import { API_URL } from "@/constants";
import { Editor } from "@monaco-editor/react";
import { createFileRoute } from "@tanstack/react-router";
import Styles from "./$branch.$.module.css";

const BlobViewer = () => {
  const content = Route.useLoaderData();

  return (
    <>
      <LocationInfo route={Route} />
      <Editor className={Styles.editor} defaultValue={content} />
    </>
  );
};

export const Route = createFileRoute("/blob/$branch/$")({
  component: BlobViewer,
  loader: async ({ params }) => {
    return fetch(`${API_URL}/blob/${params.branch}/${params._splat}`).then(
      (res) => res.text()
    );
  },
});
