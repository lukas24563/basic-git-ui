import { Column } from "primereact/column";
import { DataTable } from "primereact/datatable";
import { Skeleton } from "primereact/skeleton";
import Styles from "./files-skeleton.module.css";

const body = () => {
  const width = 50 + Math.random() * 50;
  return (
    <div style={{ width: `${width}%` }}>
      <Skeleton />
    </div>
  );
};

export const FilesSkeleton = () => {
  const rows = Array(5 + Math.floor(Math.random() * 10)).fill(1);

  return (
    <DataTable value={rows} rowClassName={() => Styles.row}>
      <Column className={Styles.nameColumn} body={body} header="Name" />
      <Column field="lastCommitMessage" body={body} header="Last commit" />
      <Column
        className={Styles.updateColumn}
        field="lastUpdate"
        body={body}
        header="Last update"
      />
    </DataTable>
  );
};
