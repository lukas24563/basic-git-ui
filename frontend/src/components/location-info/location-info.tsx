import { getRouteApi, Link, useNavigate } from "@tanstack/react-router";
import { BreadCrumb } from "primereact/breadcrumb";
import { Dropdown } from "primereact/dropdown";
import type { FunctionComponent } from "react";
import Styles from "./location-info.module.css";

type LocationInfoProps = {
  route: any;
};

export const LocationInfo: FunctionComponent<LocationInfoProps> = (props) => {
  const navigate = useNavigate({ from: props.route.to });
  const rootData = getRouteApi("__root__").useLoaderData();
  const { branch, _splat } = getRouteApi(props.route.to).useParams() as {
    branch: string;
    _splat?: string;
  };

  const hierarchy = [
    {
      label: rootData.info.name,
      to: "/tree/$branch/$",
      params: { branch, _splat: "" },
    },
  ];
  if (_splat) {
    const parts = _splat.split("/");
    parts.forEach((part, index) => {
      const path = parts.slice(0, index + 1).join("/");
      hierarchy.push({
        label: part,
        to: "/tree/$branch/$",
        params: { branch, _splat: path },
      });
    });
  }
  const breadcrumbItems = hierarchy.map((item, index) => {
    if (index === hierarchy.length - 1) {
      return { label: item.label, template: () => <span>{item.label}</span> };
    }
    return {
      label: item.label,
      template: () => (
        <Link to={item.to} params={item.params}>
          {item.label}
        </Link>
      ),
    };
  });

  const onBranchChange = (e: { value: string }) => {
    navigate({
      to: ".",
      params: (current) => ({ branch: e.value, _splat: current._splat }),
    });
  };

  return (
    <div className={Styles.container}>
      <Dropdown
        className={Styles.dropdown}
        value={branch}
        options={rootData.branches}
        onChange={onBranchChange}
      />
      <BreadCrumb model={breadcrumbItems} />
    </div>
  );
};
