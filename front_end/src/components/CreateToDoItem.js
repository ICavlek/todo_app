import React, { Component } from 'react';
import axios from "axios";

class CreateToDoItem extends Component {
  
  state = {
    title: ""
  }

  createItem = () => {

  }

  handleTitleChange = (e) => {

  }

  render() {
    return (
      <div className="inputContainer">
        <input type="text" id="name" placeholder="create to do item" value={this.state.title} onChange={this.handleTitleChange}/>
        <div className="actionButton" id="create-button" onClick={this.createItem}>
          Create
        </div>
      </div>
    )
  }
}

export default CreateToDoItem;
