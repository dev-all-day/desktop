import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { TConfig } from "../../types/config";


export type configState = {
  preferences: any;
};

const initialState: configState = {
  preferences: null
};

export const configSlice = createSlice({
  name: "config",
  initialState,
  reducers: {
    clearPreferences: (state) => {
      state.preferences = null;
    },
    setPreferences: (state,action: PayloadAction<TConfig>) => {
      // console.log(getConfig())
      // state.preferences = getConfig();
      state.preferences = action.payload;
    }

    // setShowModal: (state, action: PayloadAction<boolean>) => {
    //   // console.log(action.payload);
    //   state.showModal = action.payload;
    // },
    
  },
});

//Export actions
export const {
  clearPreferences,
  setPreferences
} = configSlice.actions;

export default configSlice.reducer;
