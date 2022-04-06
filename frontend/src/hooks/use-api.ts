import { useState } from "react";
import { ACCESS_TOKEN } from "../access_token";
import {
  IAddFoodResponse,
  IUpdateFoodRequest,
  IErrorMessage,
  IFoodRequest,
  IFoodResponse,
  IGetFoodListResponse,
  IGetUserInfoResponse,
  IGetFoodReportResponse,
  IGetUserListResponse,
  userInfoResponseToUserInfo,
} from "../messages/Food";
import { IUserInfo } from "../model/UserInfo";

export type UseApiHandler = {
  isLoading: boolean;
  errorMessage: string | null;
  getUserInfo: () => Promise<IUserInfo | null>;
  getUserList: () => Promise<IUserInfo[] | null>;
  addFood: (username: string, food: IFoodRequest) => Promise<IAddFoodResponse | null>;
  getFood: (username: string, id: String) => Promise<IFoodResponse | null>;
  updateFood: (username: string, id: String, food: IUpdateFoodRequest) => Promise<{} | null>;
  deleteFood: (username: string, id: String) => Promise<{} | null>;
  getFoodList: (username: string) => Promise<IGetFoodListResponse | null>;
  getFoodReport: () => Promise<IGetFoodReportResponse | null>;
};

export default function useApi(): UseApiHandler {
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  async function handleErrorResponse(response: Response) {
    if (response.bodyUsed) {
      const data = (await response.json()) as IErrorMessage;
      setErrorMessage(data.reason);
    } else {
      setErrorMessage(`Error: Status = ${response.status}, StatusText = ${response.statusText}`)
    }
  }

  async function getUserInfo(): Promise<IUserInfo | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/userinfo?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      let data = (await response.json()) as IGetUserInfoResponse;
      return userInfoResponseToUserInfo(data);
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  async function getUserList(): Promise<IUserInfo[] | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/user-list?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      let data = (await response.json()) as IGetUserListResponse;
      return data.users.map((userinfo: IGetUserInfoResponse) => userInfoResponseToUserInfo(userinfo));
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  async function addFood(username: string, food: IFoodRequest): Promise<IAddFoodResponse | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/user/${username}/food?access_token=${ACCESS_TOKEN}`,
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
      await handleErrorResponse(response);
      return null;
    }
  }

  async function getFood(username: string, id: String): Promise<IFoodResponse | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/user/${username}/food/${id}?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return (await response.json()) as IFoodResponse;
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  async function updateFood(username: string, id: String, food: IUpdateFoodRequest): Promise<{} | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/user/${username}/food/${id}?access_token=${ACCESS_TOKEN}`,
      {
        method: "PUT",
        body: JSON.stringify(food),
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return {};
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  async function deleteFood(username: string, id: String): Promise<{} | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/user/${username}/food/${id}?access_token=${ACCESS_TOKEN}`,
      {
        method: "DELETE",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return {};
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  async function getFoodList(username: string): Promise<IGetFoodListResponse | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/user/${username}/food?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return (await response.json()) as IGetFoodListResponse;
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  async function getFoodReport(): Promise<IGetFoodReportResponse | null> {
    setIsLoading(true);
    setErrorMessage(null);

    let response: Response = await fetch(
      `/api/food/report?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET",
      }
    );

    if (response.ok) {
      setIsLoading(false);

      return (await response.json()) as IGetFoodReportResponse;
    } else {
      setIsLoading(false);
      await handleErrorResponse(response);
      return null;
    }
  }

  return {
    isLoading,
    errorMessage,
    getUserInfo,
    getUserList,
    addFood,
    getFood,
    updateFood,
    deleteFood,
    getFoodList,
    getFoodReport,
  };
}
