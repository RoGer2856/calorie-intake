import { useState } from "react";
import { ACCESS_TOKEN } from "../access_token";
import {
  IAddFoodResponse,
  IErrorMessage,
  IFoodRequest,
  IFoodResponse,
  IGetFoodListResponse,
} from "../messages/Food";

export type UseApiHandler = {
  isLoading: boolean;
  errorMessage: string;
  addFood: (food: IFoodRequest) => Promise<IAddFoodResponse | null>;
  getFoodList: () => Promise<IGetFoodListResponse | null>;
};

export default function useApi(): UseApiHandler {
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  async function addFood(food: IFoodRequest): Promise<IAddFoodResponse | null> {
    setIsLoading(true);
    setErrorMessage("");

    let response: Response = await fetch(
      `/api/food?access_token=${ACCESS_TOKEN}`,
      {
        method: "POST",
        body: JSON.stringify(food),
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return (await response.json()) as IAddFoodResponse;
    } else {
      setIsLoading(false);

      const data = (await response.json()) as IErrorMessage;
      setErrorMessage(data.reason);
      return null;
    }
  }

  async function getFoodList(): Promise<IGetFoodListResponse | null> {
    setIsLoading(true);
    setErrorMessage("");

    let response: Response = await fetch(
      `/api/food?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return (await response.json()) as IGetFoodListResponse;
    } else {
      setIsLoading(false);

      const data = (await response.json()) as IErrorMessage;
      setErrorMessage(data.reason);
      return null;
    }
  }

  return {
    isLoading,
    errorMessage,
    addFood,
    getFoodList,
  };
}
