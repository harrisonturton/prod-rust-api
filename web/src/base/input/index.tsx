import classNames from "classnames";
import React from "react";
import styles from "./styles.module.scss";

export interface TextFieldProps {
    className?: string,
    type?: "text" | "password",
    placeholder?: string,
    value: string,
    tabIndex?: number,
    onChange: (value: string) => void,
};

export const TextField = ({ className, type="text", placeholder, tabIndex, value, onChange }: TextFieldProps) => {
    const handleInputChanged = (e: React.ChangeEvent<HTMLInputElement>) => {
        onChange(e.target.value);
        e.preventDefault();
    };

    return (
        <input
            tabIndex={tabIndex}
            className={classNames(styles.textField, className)}
            type={type}
            value={value}
            placeholder={placeholder}
            onChange={handleInputChanged}
        />    
    );
};

export interface ButtonProps {
    className?: string,
    loading?: boolean,
    disabled?: boolean,
    label: string,
    tabIndex?: number,
    onClick?: () => void,
}

export const Button = ({ className, label, loading=false, disabled=false, tabIndex, onClick }: ButtonProps) => {
    const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        onClick?.();
    }
    return (
        <button
            tabIndex={tabIndex}
            role="button"
            type="button"
            className={classNames(styles.button, className)}
            disabled={disabled || loading}
            onClick={handleClick}
        >
            {label}
        </button>
    );
};