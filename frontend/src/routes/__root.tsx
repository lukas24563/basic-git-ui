import { API_URL } from "@/constants";
import type { RepositoryInfo } from "@/types/repository-info";
import { createRootRoute, Outlet } from "@tanstack/react-router";
import Styles from "./__root.module.css";

const RootComponent = () => {
  const data = Route.useLoaderData();

  return (
    <div className={Styles.root}>
      <h1>{data.info.name}</h1>
      <Outlet />
    </div>
  );
};

export const Route = createRootRoute({
  component: RootComponent,
  loader: async () => {
    const info = fetch(API_URL + "/info").then((res) => res.json());
    const branches = fetch(API_URL + "/branches").then((res) => res.json());
    return {
      info: (await info) as RepositoryInfo,
      branches: await branches,
    };
  },
});
