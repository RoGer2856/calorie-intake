import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { IFoodResponse } from "../messages/Food";
import { addFoodToAll, AllFoods, createAllFoods } from "../model/Foods";

export interface IMyFoodsState {
	foods: AllFoods,
}

const initialState = {
	foods: createAllFoods(),
};

const slice = createSlice({
	name: "UserInfo",
	initialState: initialState,
	reducers: {
		set(state: IMyFoodsState, action: PayloadAction<AllFoods>) {
			state.foods = action.payload;
		},
		add(state: IMyFoodsState, action: PayloadAction<IFoodResponse>) {
			addFoodToAll(state.foods, action.payload);
		}
	}
});

export const myFoodsSliceReducer = slice.reducer;
export const myFoodsActions = slice.actions;
