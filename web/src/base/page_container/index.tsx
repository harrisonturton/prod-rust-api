import styles from "./styles.module.scss";

export interface PageContainerProps {
    children: React.ReactNode
}

const PageContainer = ({ children }: PageContainerProps) => (
    <main className={styles.root}>
        {children}
    </main>
);

export default PageContainer;