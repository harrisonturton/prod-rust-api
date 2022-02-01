import classNames from "classnames";
import * as styles from "./styles.module.scss";

const FileTabs = () => (
    <div className={styles.toolbar}>
        <Tab label="README.md" />
        <Tab label="export.csv" active={true} />
        <Tab label="notebook.ipnyb" />
    </div>
);

interface TabProps {
    label: string;
    active?: boolean;
}

const Tab = ({ label, active }: TabProps) => (
    <button className={classNames(styles.menu, { [styles.active]: active })}>
        {label}
    </button>
);

export default FileTabs;
