import { configureStore } from "@reduxjs/toolkit";
import { combineReducers } from "redux";

import {
  connectionsReducer,
  configReducer
} from "./slices";

const rootReducer = combineReducers({
  connections: connectionsReducer,
  config: configReducer
});

const store = configureStore({
  reducer: rootReducer
})

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
// export type AppThunk<ReturnType = void> = ThunkAction<ReturnType, RootState, unknown, Action<string>>;

export default store;