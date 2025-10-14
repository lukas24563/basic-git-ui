import { createFileRoute, getRouteApi, Navigate } from "@tanstack/react-router";
import { Route as TreeRoute } from "./tree/$branch.$";

const App = () => {
  const info = getRouteApi("__root__").useLoaderData().info;

  if (info.main_branch === undefined) return <h1>No main branch set</h1>;

  return <Navigate to={TreeRoute.to} params={{ branch: info.main_branch }} />;
};

export const Route = createFileRoute("/")({
  component: App,
});
