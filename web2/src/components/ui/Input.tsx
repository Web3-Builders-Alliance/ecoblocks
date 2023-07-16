import clsx from 'clsx';
import React, { forwardRef } from 'react';

interface InputProps extends React.HTMLProps<HTMLInputElement> {
  name: string;
  id: string;
  type?: string;
  className?: string;
  required?: boolean;
  label?: string;
  placeholder?: string;
  error?: string;
  disabled?: boolean;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      name,
      id,
      type = 'text',
      className,
      required = false,
      label = '',
      placeholder = '',
      error = '',
      disabled = false,
      ...otherProps
    },
    ref
  ) => (
    <div className="relative flex flex-col">
      <div className="form-control w-full max-w-xs">
        {label && (
          <label htmlFor={id} className={clsx('label')}>
            <span className="label-text">
              {label} {required && <span> *</span>}
            </span>
          </label>
        )}
        <input
          type={type}
          id={id}
          name={name}
          className={clsx('input input-bordered w-full max-w-xs', error && 'input-error')}
          placeholder={placeholder}
          required={required}
          disabled={disabled}
          ref={ref}
          {...otherProps}
        ></input>
        {error && (
          <label className="label">
            <span className="label-text-alt">{error}</span>
          </label>
        )}
      </div>
    </div>
  )
);

Input.displayName = 'Input';
export default Input;
