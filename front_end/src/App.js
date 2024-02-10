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
    axios.get(
      "http://127.0.0.1:8000/v1/item/get",
      {headers: {"token": "some_token"}}
    ).then(
      response => {
        let pending_items = response.data["pending_items"];
        let done_items = response.data["done_items"];
        this.setState({
          "pending_items": this.processItemValues(pending_items),
          "done_items": this.processItemValues(done_items),
          "pending_items_count": response.data["pending_item_count"],
          "done_items_count": response.data["done_item_count"]
        });
      }
    )
  };

  // ensures the API call is updated when mounted
  componentDidMount() {
    this.getItems();
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
