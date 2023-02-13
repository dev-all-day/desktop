import { createSlice, PayloadAction } from "@reduxjs/toolkit";


export type connectionsState = {
  connections: any;
};

const initialState: connectionsState = {
  connections: null
};

export const connectionsSlice = createSlice({
  name: "connections",
  initialState,
  reducers: {
    clearConnections: (state) => {
      state.connections = null;
    },

    // setShowModal: (state, action: PayloadAction<boolean>) => {
    //   // console.log(action.payload);
    //   state.showModal = action.payload;
    // },
    
  },
});

//Export actions
export const {
    clearConnections
} = connectionsSlice.actions;

export default connectionsSlice.reducer;
