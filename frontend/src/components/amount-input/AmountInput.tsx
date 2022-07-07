import { TextField } from '@mui/material'
import React, { useState } from 'react'
import './AmountInput.scss'

type AmountInputProps = {
  label?: string,
  value?: string,
  minValue?: number,
  disabled?: boolean,
  onSetValidAmount: (value: string) => void
}

export const AmountInput = (props: AmountInputProps) => {
  const [value, setValue] = useState(props.value);
  const [error, setError] = useState({ hasError: false, message: '' });

  const handleOnTextChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      const _error = {
          hasError: false,
          message: ''
      };
      
      setValue(event.target.value);

      if (!_error.hasError) {
          props.onSetValidAmount(event.target.value);
      }
      setError(_error);
  };
  
  return (
    <TextField className='AmountInput'
      type='number'
      placeholder='0.000000'
      disabled={props.disabled}
      value={value}
      label={props.label}
      error={error.hasError}
      onChange={handleOnTextChange}
      helperText={error.message}></TextField>
  )
}
