import { SpinnerCircular } from "spinners-react";
import styles from "./styles.module.scss";

export const Spinner = () => (
    <div className={styles.spinner}>
        <SpinnerCircular
            size="100%"
            thickness={200}
            speed={200}
            secondaryColor="transparent"
        />
    </div>
);
