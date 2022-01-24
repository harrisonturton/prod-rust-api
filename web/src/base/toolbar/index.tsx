import * as styles from "./styles.module.scss";

const Toolbar = () => (
    <div className={styles.toolbar}>
        <Menu label="Home"/>
        <Menu label="File"/>
        <Menu label="Tools"/>
    </div>
);

const Menu = ({ label }: { label: string }) => (
    <button className={styles.menu}>
        {label}
    </button>
);

export default Toolbar;