import { CommitChanges } from "@/components/commit-changes/commit-changes";
import { LocationInfo } from "@/components/location-info/location-info";
import { API_URL } from "@/constants";
import type { CommitDetails } from "@/types/commit-details";
import { Editor } from "@monaco-editor/react";
import { createFileRoute, useBlocker, useRouter } from "@tanstack/react-router";
import { useState } from "react";
import Styles from "./$branch.$.module.css";

const BlobViewer = () => {
  const content = Route.useLoaderData();
  const [changes, setChanges] = useState<string | undefined>();
  const params = Route.useParams();
  const router = useRouter();
  const match = Route.useMatch();

  useBlocker({
    shouldBlockFn: () => {
      if (changes === undefined) {
        return false;
      }

      const confirmLeave = confirm(
        "You have unsaved changes. Are you sure you want to leave?"
      );
      return !confirmLeave;
    },
    enableBeforeUnload: changes !== undefined,
  });

  const onChange = (value: string | undefined) => {
    if (value === content) {
      setChanges(undefined);
      return;
    }

    setChanges(value);
  };

  const onPushChanges = async (details: CommitDetails) => {
    const response = await fetch(
      `${API_URL}/blob/${encodeURIComponent(params.branch)}/${params._splat}`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ ...details, content: changes }),
      }
    );

    if (response.ok) {
      router.invalidate({
        filter: (route) => route.id === match.id,
      });
      setChanges(undefined);
      return true;
    }
    return false;
  };

  return (
    <>
      <LocationInfo route={Route} />
      <Editor
        wrapperProps={{ className: Styles.wrapper }}
        className={Styles.editor}
        defaultValue={content}
        onChange={onChange}
      />
      <CommitChanges changes={changes} onPushChanges={onPushChanges} />
    </>
  );
};

export const Route = createFileRoute("/blob/$branch/$")({
  component: BlobViewer,
  loader: async ({ params }) => {
    return fetch(
      `${API_URL}/blob/${encodeURIComponent(params.branch)}/${params._splat}`
    ).then((res) => res.text());
  },
});
