import classNames from "classnames";
import React from "react";
import styles from "./styles.module.scss";

export interface TextFieldProps {
    className?: string;
    type?: "text" | "password";
    placeholder?: string;
    valid?: boolean;
    value: string;
    tabIndex?: number;
    required?: boolean;
    autoFocus?: boolean;
    onChange: (value: string) => void;
}

export const TextField = ({
    className,
    type = "text",
    placeholder,
    tabIndex,
    value,
    valid = true,
    required,
    autoFocus,
    onChange,
}: TextFieldProps) => {
    const handleInputChanged = (e: React.ChangeEvent<HTMLInputElement>) => {
        onChange(e.target.value);
        e.preventDefault();
    };
    const textfieldClassNames = classNames(styles.textField, className, {
        [styles.invalid]: !valid,
    });
    return (
        <input
            required={required}
            tabIndex={tabIndex}
            className={textfieldClassNames}
            type={type}
            value={value}
            placeholder={placeholder}
            autoFocus={autoFocus}
            onChange={handleInputChanged}
        />
    );
};
