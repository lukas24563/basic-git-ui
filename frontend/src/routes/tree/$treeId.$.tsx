import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/tree/$treeId/$")({
  component: RouteComponent,
});

function RouteComponent() {
  return <div>Hello "/tree/$treeId"!</div>;
}
