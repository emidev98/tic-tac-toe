import React, { useState } from 'react'
import './AddressInput.scss'
import { TextField } from '@mui/material';
import { InputLabel } from '@mui/material';
import { bech32 } from "bech32";

type AddressInputProps = {
    label?: string,
    value?: string,
    disabled?: boolean,
    onSetValidAddress: (value: string) => void
}

export const AddressInput = (props: AddressInputProps) => {
    const [value, setValue] = useState(props.value);
    const [error, setError] = useState({ hasError: false, message: '' });

    const handleOnTextChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const _error = {
            hasError: false,
            message: ''
        };

        try {
            const { prefix: decodedPrefix } = bech32.decode(event.target.value);
            _error.hasError = decodedPrefix !== "terra";
            _error.message = _error.hasError ? 'Invalid address' : '';
        }
        catch (e: any) {
            _error.hasError = true;
            _error.message = e.message;
        }
        setValue(event.target.value);

        if (!_error.hasError) {
            props.onSetValidAddress(event.target.value);
        }
        setError(_error);
    };

    return (
        <TextField className='AddressInput'
            placeholder='terra...'
            disabled={props.disabled}
            value={value}
            label={props.label}
            error={error.hasError}
            onChange={handleOnTextChange}
            helperText={error.message}></TextField>
    )
}
