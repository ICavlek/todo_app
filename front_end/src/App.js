import React, { Component } from 'react';
import axios from 'axios';

class App extends Component {
  state = {
    "pending_items": [],
    "done_items": [],
    "pending_items_count": 0,
    "done_items_count": 0
  }

  // makes the API call
  getItems() {

  }

  // ensures the API call is updated when mounted
  componentDidMount() {

  }

  // convert items from API to HTML
  processItemValues(items) {

  }

  // returns the HTML to be rendered
  render() {
    return (
      <div className="App">
      <p>To do application</p>
      </div>
    )
  }
}

export default App;
