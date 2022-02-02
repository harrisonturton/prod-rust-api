import { Spinner } from "base/spinner";
import classNames from "classnames";
import styles from "./styles.module.scss";

export type ButtonVariant
    = "primary"
    | "secondary"
    | "tertiary";

export interface ButtonProps {
    variant?: ButtonVariant,
    loading?: boolean;
    disabled?: boolean;
    label: string;
    tabIndex?: number;
    autoFocus?: boolean;
    onClick?: () => void;
}

export const Button = ({
    variant="primary",
    label,
    loading = false,
    disabled = false,
    autoFocus,
    tabIndex,
    onClick,
}: ButtonProps) => {
    const handleClick = (ev: React.MouseEvent<HTMLButtonElement>) => {
        onClick?.();
    };
    return (
        <button
            tabIndex={tabIndex}
            role="button"
            type="button"
            autoFocus={autoFocus}
            className={classNames(styles.button, {
                [styles.loading]: loading,
                [styles.primary]: variant == "primary",
                [styles.secondary]: variant == "secondary",
                [styles.tertiary]: variant == "tertiary",
            })}
            disabled={disabled || loading}
            onClick={handleClick}
        >
            {loading ? <Spinner /> : label}
        </button>
    );
};
