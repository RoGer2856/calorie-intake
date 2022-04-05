import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { IFoodResponse } from "../messages/Food";
import { addFoodToAll, IAllFoods, createAllFoods } from "../model/Foods";

export interface IMyFoodsState {
	foods: IAllFoods,
}

const initialState = {
	foods: createAllFoods(null),
};

const slice = createSlice({
	name: "UserInfo",
	initialState: initialState,
	reducers: {
		set(state: IMyFoodsState, action: PayloadAction<IAllFoods>) {
			state.foods = action.payload;
		},
		add(state: IMyFoodsState, action: PayloadAction<IFoodResponse>) {
			addFoodToAll(state.foods, action.payload);
		}
	}
});

export const myFoodsSliceReducer = slice.reducer;
export const myFoodsActions = slice.actions;
