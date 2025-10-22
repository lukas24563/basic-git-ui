import type { CommitDetails } from "@/types/commit-details";
import { Button } from "primereact/button";
import { Dialog } from "primereact/dialog";
import { Divider } from "primereact/divider";
import { FloatLabel } from "primereact/floatlabel";
import { InputText } from "primereact/inputtext";
import { useState, type FunctionComponent } from "react";
import Styles from "./commit-changes.module.css";

type CommitChangesProps = {
  changes?: string;
  onPushChanges: (details: CommitDetails) => Promise<boolean>;
};

export const CommitChanges: FunctionComponent<CommitChangesProps> = (props) => {
  const [showDialog, setShowDialog] = useState(false);
  const [commitDetails, setCommitDetails] = useState<CommitDetails>({
    message: "",
    name: "",
    email: "",
  });

  const buttonClasses = [Styles.button];
  if (props.changes) {
    buttonClasses.push(Styles.visible);
  }

  const formComplete = Object.values(commitDetails).every(
    (value) => value.length > 0
  );

  const onPushChanges = async () => {
    const success = await props.onPushChanges(commitDetails);
    if (success) {
      setShowDialog(false);
    }
  };

  const getDialogContent = () => (
    <>
      <FloatLabel>
        <InputText
          id="message"
          value={commitDetails.message}
          onChange={(e) =>
            setCommitDetails((current) => ({
              ...current,
              message: e.target.value,
            }))
          }
        />
        <label htmlFor="message">Message</label>
      </FloatLabel>
      <Divider className={Styles.divider}>Author information</Divider>
      <FloatLabel>
        <InputText
          id="name"
          value={commitDetails.name}
          onChange={(e) =>
            setCommitDetails((current) => ({
              ...current,
              name: e.target.value,
            }))
          }
        />
        <label htmlFor="name">Name</label>
      </FloatLabel>
      <FloatLabel>
        <InputText
          id="email"
          value={commitDetails.email}
          onChange={(e) =>
            setCommitDetails((current) => ({
              ...current,
              email: e.target.value,
            }))
          }
        />
        <label htmlFor="email">Email</label>
      </FloatLabel>
      <Button
        disabled={!formComplete}
        label="Commit and push"
        onClick={onPushChanges}
      />
    </>
  );

  return (
    <div className={Styles.container}>
      <Button
        className={buttonClasses.join(" ")}
        label="Commit changes"
        onClick={() => setShowDialog(true)}
      />
      <Dialog
        className={Styles.dialog}
        header="Edit commit"
        visible={showDialog}
        onHide={() => setShowDialog(false)}
        contentClassName={Styles.dialogContent}
      >
        {getDialogContent()}
      </Dialog>
    </div>
  );
};
