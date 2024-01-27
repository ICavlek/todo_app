let createButton = document.getElementById("create-button");
createButton.addEventListener("click", postAlert);

function postAlert() {
  let titleInput = document.getElementById("name");
  alert(titleInput.value);
  titleInput.value = null;
}

function renderItems(items, processType, elementId, processFunction) {
  let itemsMeta = [];
  let placeholder = "<div>"

  for (let i = 0; i < items.length; i++) {
    let title = items[i]["title"];
    let placeholderId = processType + "-" + title.replaceAll(" ", "-");
    placeholder += "<div>" + title + "<button " + 'id="' + placeholderId + '">' + processType + '</button>' + "</div>";
    itemsMeta.push({"id": placeholderId, "title": title});
  }

  placeholder += "</div>"
  document.getElementById(elementId).innerHTML = placeholder;

  for (let i = 0; i < itemsMeta.length; i++) {
    document.getElementById(itemsMeta[i]["id"]).addEventListener("click", processFunction);
  }
}
