import { useState } from "react";

export type UseInputHandler = {
  value: string,
  isValid: boolean,
  hasError: boolean,
  valueChangeHandler: (event: React.FormEvent<HTMLInputElement>) => void,
  inputBlurHandler: (event: React.FormEvent<HTMLInputElement>) => void,
  reset: (value: string) => void,
};

export default function useInput(
  defaultValue: string,
  validateValue: (value: string) => boolean
): UseInputHandler {

  const [enteredValue, setEnteredValue] = useState<string>(defaultValue);
  const [isTouched, setIsTouched] = useState<boolean>(false);

  const valueIsValid = enteredValue ? validateValue(enteredValue) : false;
  const hasError = !valueIsValid && isTouched;

  function valueChangeHandler(event: React.FormEvent<HTMLInputElement>) {
    setEnteredValue(event.currentTarget.value);
  }

  function inputBlurHandler(event: React.FormEvent<HTMLInputElement>) {
    setIsTouched(true);
  }

  function reset(value: string) {
    setEnteredValue(value);
    setIsTouched(false);
  }

  const ret: UseInputHandler = {
    value: enteredValue,
    isValid: valueIsValid,
    hasError: hasError,
    valueChangeHandler: valueChangeHandler,
    inputBlurHandler: inputBlurHandler,
    reset: reset,
  };

  return ret;
}
