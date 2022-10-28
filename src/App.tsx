import { BinarySearchTree, useBinarySearchTree } from "react-tree-vis";
import { useState } from "react";
import "./App.css";

export default function App() {
  const { ref, insert, remove } = useBinarySearchTree();
  const [insertValue, setInsertValue] = useState(0);
  const [removeValue, setRemoveValue] = useState(0);

  return (
    <div className="MainContainer">
      <BinarySearchTree
        data={[2, 1, 3]}
        ref={ref}
        treeStyles={{ nodeShadow: "0 0 1px 1px #0000000" }}
      />
      <div className="inputcontainer">
        <div className="container">
          <input
            type="number"
            onChange={(elem) =>
              setInsertValue(parseInt(elem.currentTarget.value, 10))
            }
          />
          <button onClick={() => insert(insertValue)}>Insert</button>
        </div>

        <div className="container">
          <input
            type="number"
            onChange={(elem) =>
              setRemoveValue(parseInt(elem.currentTarget.value, 10))
            }
          />
          <button onClick={() => remove(removeValue)}>Remove</button>
        </div>
      </div>
    </div>
  );
}
