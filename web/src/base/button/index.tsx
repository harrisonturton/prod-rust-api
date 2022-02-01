import { Spinner } from "base/spinner";
import classNames from "classnames";
import styles from "./styles.module.scss";

export interface ButtonProps {
    className?: string;
    loading?: boolean;
    disabled?: boolean;
    label: string;
    tabIndex?: number;
    autoFocus?: boolean;
    onClick?: () => void;
}

export const Button = ({
    className,
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
            className={classNames(styles.button, className, {
                [styles.loading]: loading,
            })}
            disabled={disabled || loading}
            onClick={handleClick}
        >
            {loading ? <Spinner /> : label}
        </button>
    );
};
