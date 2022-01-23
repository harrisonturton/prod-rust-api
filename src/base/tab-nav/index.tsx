import * as styles from "./styles.module.scss";

const TabNav = () => (
    <div className={styles.root}>
        <Tab/>
        <Tab/>
        <Tab/>
        <Tab/>
        <Tab/>
        <Tab/>
    </div>
);

const Tab = () => (
    <button className={styles.tab}>

    </button>
);

export default TabNav;