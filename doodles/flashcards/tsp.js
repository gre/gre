// source https://codepen.io/jakealbaugh/pen/PNxypP

////////////////////////////////////////////////////////////
// Travelling Salesman Solver
// hoping to do the impossible.
// Jake Albaugh | @jake_albaugh | jakealbaugh.com
////////////////////////////////////////////////////////////

// 2021 forked by @greweb

export function TravellingSalesmanSolver(test, onProgress, notify) {
  var TSS = {},
    UTIL = {};

  init();

  return TSS;

  ////////////////////
  // INITIALIZE
  ////////////////////

  async function init() {
    setUtil();
    setData();
    setNodeData();
    setNodes();
    passFirst();
    passIO();
  }

  ////////////////////
  // PASSES
  ////////////////////

  // draw two valid connections from the furthest node in
  function passFirst() {
    // find two closest nodes, starting from furthest
    for (var n = 1; n < TSS.nodes.length; n++) {
      // grab the current node
      var node = TSS.nodes[n];

      // tell the world
      TSS.notify("Node #" + node.id);

      // if this node has already been hooked up as a receiver
      if (nodeIsConnected(node)) {
        TSS.notify("OK");
        continue;
      }
      // proximity nodes
      var prox_nodes = TSS.nodes.slice();
      prox_nodes.shift();
      prox_nodes.sort(UTIL.sortByDistanceToNode(node));

      // set our initial closest values for the two stems
      var closest_1 = false,
        closest_2 = false;
      // for each proximity-sorted node
      for (var i = 0; i < prox_nodes.length; i++) {
        // if it is connected, move on
        if (nodeIsConnected(node)) break;
        // if we have already found what we need, move on
        if (closest_1 && closest_2) break;

        // get the edge node candidate
        var edge_cdt = prox_nodes[i];

        // if it is checking itself, go to the next one (wreck yourself)
        if (node.id === edge_cdt.id) continue;
        // if the prospect is taken, go to next one
        if (nodeIsConnected(edge_cdt)) continue;

        // if matchable
        if (noMatch(node, edge_cdt)) {
          // if we havent defined the first closest
          if (!closest_1) {
            closest_1 = true;
            // set relationship with node as sender
            setRelationship(node, edge_cdt);
            // if we havent defined the second closest
          } else if (!closest_2) {
            closest_2 = true;
            // set relationship with edge_cdt as sender
            setRelationship(edge_cdt, node);
            // we have what we need, update our datum
          } else {
            // TODO: this doesnt happen???
            console.warn("this firing is unexpected");
            break;
          }
        }
      }
    }

    // find the last two nodes that arent connected and connect them
    var last_nodes = [];
    for (var i = 1; i < TSS.nodes.length; i++) {
      var node = TSS.nodes[i];
      if (!nodeIsConnected(node)) last_nodes.push(node);
      if (last_nodes.length === 2) break;
    }

    var last_0 = last_nodes[0],
      last_1 = last_nodes[1];
    setRelationship(last_1, last_0);

    // TSS.nodes[last_1.id].tmp_snd.push(last_0.id);
    // TSS.nodes[last_0.id].tmp_rec.push(last_1.id);
  }

  // loop through all nodes and create sending/receiving pairs.
  async function passIO() {
    var count = TSS.nodes.length - 1;
    var traversed = 0;
    var curr_node = TSS.nodes[1];
    // initial rev node will not have an id match.
    // this gets set each loop
    var prev_node = { id: "not a chance" };

    // start at closest node to center
    while (traversed < count) {
      // add current node id to path
      TSS.path.push(curr_node.id);
      // get the next point
      var next_node = getNextNodeInPath(curr_node, prev_node);
      if (next_node) {
        // TODO: this condition shouldnt be necessary
        // set the master send and receive values
        curr_node.send_to = next_node.id;
        next_node.receive_from = curr_node.id;
        // set previous node for next loop
        prev_node = curr_node;
        // set current node for next loop
        curr_node = next_node;
      } else {
        console.warn("we here", curr_node.tmp_snd.concat(curr_node.tmp_rec));
      }
      // increase traversed count
      traversed++;
    }

    // now we have our path.
    // step through the path, grab 5 points forward, determine shortest route for 2,3,4 to get between 1 and 5.
    // if so, update the values
    TSS.path = await traverseArray(TSS.path, 0);
    TSS.notify("Generated Path", TSS.path);

    // very important. recalculate distance.
    for (var i = 1; i < TSS.nodes.length - 1; i++) {
      var node = TSS.nodes[i];
      if (node.send_to) {
        TSS.data.distances.optimized += UTIL.distanceToCenter(
          node,
          TSS.nodes[node.send_to]
        );
      }
    }

    // at this point, TSS.data.chains should be an array containing one array containing all of our values
    if (TSS.data.chains[0].length !== TSS.data.node_count + 1)
      console.warn("chain logic is broken");
  }

  ////////////////////
  // SETTERS
  ////////////////////

  // set utility functions
  function setUtil() {
    TSS.notify = notify || (() => {});

    UTIL.distanceForSet = utilDistanceForSet;
    UTIL.distanceToCenter = utilDistanceToCenter;

    UTIL.formatInt = utilFormatInt;
    UTIL.formatTime = utilFormatTime;
    UTIL.formatLocaleString = utilFormatLocaleString;

    UTIL.permutations = utilPermutations;

    UTIL.sortByDistanceToCenter = utilSortByDistanceToCenter;
    UTIL.sortByDistanceToNode = utilSortByDistanceToNode;
    UTIL.sortByFirstValue = utilSortByFirstValue;
  }

  // set initial empty data
  function setData() {
    TSS.test = test;
    TSS.path = [];
    TSS.nodes = [];
    TSS.data = {
      chains: [],
      dimensions: { w: 0, h: 0 },
      distances: {
        first: 0,
        optimized: 0,
      },
      dot: {},
      node_count: 0,
      path_scrub: 6, // +1 = how many at a time
      path_step: 3, // how many to jump ahead. 1 is super cray, but half scrub is pretty good
      scale: TSS.test.scale || 10,
      timer: {
        start: new Date().getTime(),
        elapsed: null,
        saved: null,
      },
    };
    TSS.data.dot.di = 0.8 * TSS.data.scale;
    TSS.data.dot.rad = TSS.data.dot.di / 2;
  }

  // set initial data from test point
  function setNodeData() {
    // set the dimensions
    for (var i = 0; i < TSS.test.points.length; i++) {
      if (TSS.test.points[i][0] * TSS.data.scale > TSS.data.dimensions.w)
        TSS.data.dimensions.w = TSS.test.points[i][0] * TSS.data.scale;
      if (TSS.test.points[i][1] * TSS.data.scale > TSS.data.dimensions.h)
        TSS.data.dimensions.h = TSS.test.points[i][1] * TSS.data.scale;
    }
    // add dot diameter to both dimensions
    TSS.data.dimensions.h += TSS.data.dot.di;
    TSS.data.dimensions.w += TSS.data.dot.di;

    // set the count
    TSS.data.node_count = TSS.test.points.length - 1;
  }

  // set our nodes
  function setNodes() {
    // generating test nodes data
    var nodes = [];
    for (var i = 0; i < TSS.test.points.length; i++) {
      // rel values are plots rel to top left
      var abs_x = TSS.test.points[i][0] * TSS.data.scale + TSS.data.dot.rad,
        abs_y = TSS.test.points[i][1] * TSS.data.scale + TSS.data.dot.rad;
      // if data has reverse true for the axis, switch it
      if (TSS.test.rev_x) abs_x = TSS.data.dimensions.w - abs_x;
      if (TSS.test.rev_y) abs_y = TSS.data.dimensions.h - abs_y;
      // rel values are plots rel to center
      var rel_x = TSS.data.dimensions.w / -2 + abs_x,
        rel_y = TSS.data.dimensions.h / -2 + abs_y;

      // this doesnt get used right now,
      // but if we need clockwise / counter logic later on, this helps
      var dir;
      if (rel_x <= 0 && rel_y <= 0) dir = "nw";
      if (rel_x > 0 && rel_y <= 0) dir = "ne";
      if (rel_x > 0 && rel_y > 0) dir = "se";
      if (rel_x <= 0 && rel_y > 0) dir = "sw";

      nodes.push({
        id: null,
        dir: dir,
        tmp_snd: [],
        tmp_rec: [],
        send_to: null,
        receive_from: null,
        abs_x: abs_x,
        abs_y: abs_y,
        rel_x: rel_x,
        rel_y: rel_y,
        // generate the distance from center once
        dist_c: UTIL.distanceToCenter(
          { rel_x: rel_x, rel_y: rel_y },
          { rel_x: 0, rel_y: 0 }
        ),
      });
    }

    // sort nodes by proximity to center, furthest first
    nodes.sort(UTIL.sortByDistanceToCenter);

    // assign ids, colors, and flag origin
    for (var p = 0; p < nodes.length; p++) {
      nodes[p].id = p + 1;
      if (p === nodes.length - 1) {
        nodes[p].color = "black";
        nodes[p].is_origin = true;
      } else {
        var rat = p / (nodes.length - 1);
        var hue = Math.floor(Math.random() * 360);
        nodes[p].color =
          "hsl(" + hue + ", 70%, " + (0.4 + rat * 0.6) * 80 + "%)";
        nodes[p].is_origin = false;
      }
    }

    // apply the nodes with an empty first
    TSS.nodes = ["BLANK"].concat(nodes);
  }

  ////////////////////
  // APP METHODS
  ////////////////////

  // return the next node, in send => receive priority order
  function getNextNodeInPath(curr, prev) {
    // concatenate the current nodes connections
    var arr = curr.tmp_snd.concat(curr.tmp_rec);
    // grab the first id that isnt this node's sender
    for (var i = 0; i < arr.length; i++) {
      if (arr[i] !== prev.id) return TSS.nodes[arr[i]];
    }
  }

  // determining if a node has both edges connected
  function nodeIsConnected(node) {
    if (node.tmp_snd.length + node.tmp_rec.length > 1) return true;
    return false;
  }

  // relate two sender to each other
  function setRelationship(sender, receiver) {
    // update the distance traveled
    TSS.data.distances.first += UTIL.distanceToCenter(sender, receiver);

    // tell the world
    TSS.notify(sender.id + "->" + receiver.id);

    // set on sender
    sender.tmp_snd.push(receiver.id);
    // set on receiver
    receiver.tmp_rec.push(sender.id);

    // figuring out which if any chain for sender and receiver
    var snd_chain = null;
    var rec_chain = null;

    // finding chains for the parent and the prospect sender
    for (var i = 0; i < TSS.data.chains.length; i++) {
      // checking for a parent chain
      if (TSS.data.chains[i].indexOf(sender.id) !== -1) {
        snd_chain = TSS.data.chains[i];
        if (rec_chain) break;
      }
      // checking for a prospect chain
      if (TSS.data.chains[i].indexOf(receiver.id) !== -1) {
        rec_chain = TSS.data.chains[i];
        if (snd_chain) break;
      }
    }

    // if sender chain and no receiver chain, add receiver to sender chain
    var snd_index, rec_index;
    if (snd_chain && !rec_chain) {
      snd_index = TSS.data.chains.indexOf(snd_chain);
      TSS.data.chains[snd_index] = snd_chain.concat([receiver.id]);
      // if receiver chain and no sender chain, add sender to receiver chain
    } else if (!snd_chain && rec_chain) {
      rec_index = TSS.data.chains.indexOf(rec_chain);
      TSS.data.chains[rec_index] = rec_chain.concat([sender.id]);
      // if no sender or receiver chain, add a new chain
    } else if (!snd_chain && !rec_chain) {
      TSS.data.chains.push([sender.id, receiver.id]);
      // if sender chain is receiver chain more than for the last nodes,
      // our logic that got us here is broken
    } else if (snd_chain === rec_chain) {
      TSS.notify("Chain Match: should happen once for last nodes");
      // if sender chain is not the receiver chain, merge the TSS.data.chains
    } else {
      // we have two different TSS.data.chains
      snd_index = TSS.data.chains.indexOf(snd_chain);
      // merge the two TSS.data.chains
      snd_chain = snd_chain.concat(rec_chain);
      // set the index
      TSS.data.chains[snd_index] = snd_chain;
      // remove the receiver chain now that it is merged
      TSS.data.chains.splice(TSS.data.chains.indexOf(rec_chain), 1);
    }
  }

  // determining if a node qualifies as a match to a parent node
  function noMatch(node1, node2) {
    // if already a match, ignore
    if (
      node1.tmp_snd.indexOf(node2.id) !== -1 ||
      node2.tmp_snd.indexOf(node1.id) !== -1 ||
      node1.tmp_rec.indexOf(node2.id) !== -1 ||
      node2.tmp_rec.indexOf(node1.id) !== -1
    )
      return false;

    // TODO: logic, prevent complete loops
    // if chain proximity, trying to prevent loops
    var same_chain = false;
    for (var c = 0; c < TSS.data.chains.length; c++) {
      var chain = TSS.data.chains[c];
      if (chain.indexOf(node2.id) !== -1 && chain.indexOf(node1.id) !== -1) {
        same_chain = true;
        break;
      }
    }
    if (same_chain) return false;

    // if center node has one already
    if (node1.is_origin && node1.tmp_snd.length + node1.tmp_rec.length > 1)
      return false;
    if (node2.is_origin && node2.tmp_snd.length + node2.tmp_rec.length > 1)
      return false;

    // pass
    return true;
  }

  // traverse the path array from a given point
  async function traverseArray(arr, i) {
    // make a modifiable copy
    var tmp_arr = arr.slice();

    var path_i = arr[i];
    var start = TSS.nodes[path_i];
    var end_idx = Math.min(i + TSS.data.path_scrub, arr.length - 1);

    // tell the world
    // assholes are knots in ropes in sailing.
    // that is exactly what we are doing here.
    // thanks Thomas Horton for the tidbit.
    TSS.notify("AssBlast P#" + i + "-" + end_idx);

    var ok_scrub = end_idx - i;
    var end = TSS.nodes[arr[end_idx]];
    var betweens = [];
    // generate between items
    for (var b = 1; b < ok_scrub; b++) {
      betweens.push(TSS.nodes[arr[i + b]]);
    }
    var perms = UTIL.permutations(betweens);
    var d = Infinity;
    var match = null;

    // for each permutation, get its distance
    for (var p = 0; p < perms.length; p++) {
      var dist = UTIL.distanceForSet(start, end, perms[p]);
      // if best distance, set it
      if (dist < d) {
        d = dist;
        match = p;
      }
    }

    var compare_1 = perms[match]
      .map(function (a) {
        return a.id;
      })
      .join("-");
    var compare_2 = betweens
      .map(function (a) {
        return a.id;
      })
      .join("-");

    // if different
    if (compare_1 !== compare_2) {
      // redefine based on match
      var matching = perms[match];
      var prev = start;
      for (var t = 0; t < matching.length; t++) {
        var m = matching[t];
        tmp_arr[i + t + 1] = m.id;
        var node = TSS.nodes[m.id];
        // set the send of pref
        TSS.nodes[prev.id].send_to = node.id;
        // set the receive for current loop
        TSS.nodes[node.id].receive_from = prev.id;
        prev = TSS.nodes[node.id];
      }
      var last = matching[matching.length - 1];
      TSS.nodes[end.id].receive_from = last.id;
      TSS.nodes[last.id].send_to = end.id;
    }

    if (!(await onProgress(tmp_arr))) return tmp_arr;

    // if still going, call again with modified array
    if (i < arr.length - TSS.data.path_scrub) {
      const r = await traverseArray(tmp_arr, i + TSS.data.path_step);
      return r;
      // if note going, set path
    } else {
      return tmp_arr;
    }
  }

  ////////////////////
  // UTILS
  ////////////////////

  // get computed distance for a series of nodes
  function utilDistanceForSet(start, end, set) {
    var d = 0;
    var curr = start;
    for (var i = 0; i < set.length; i++) {
      d += utilDistanceToCenter(curr, set[i]);
      curr = set[i];
    }
    d += utilDistanceToCenter(set[set.length - 1], end);
    return d;
  }

  // get distance between two nodes
  function utilDistanceToCenter(node_1, node_2) {
    var a = node_1.rel_x - node_2.rel_x;
    var b = node_1.rel_y - node_2.rel_y;
    return Math.sqrt(a * a + b * b);
  }

  // calling integer format
  function utilFormatInt(int) {
    return utilFormatLocaleString(int);
  }

  // nice looking integers
  function utilFormatLocaleString(x, sep, grp) {
    var sx = ("" + x).split("."),
      s = "",
      i,
      j;
    sep = sep || ",";
    grp = grp || grp === 0 ? grp : grp + 3;
    i = sx[0].length;
    while (i > grp) {
      j = i - grp;
      s = sep + sx[0].slice(j, i) + s;
      i = j;
    }
    s = sx[0].slice(0, i) + s;
    sx[0] = s;
    return sx.join(".");
  }

  // nice looking time
  function utilFormatTime(ms) {
    if (Math.floor(ms / 3600000) > 0)
      return UTIL.formatInt(Math.round((ms / 3600000) * 100) / 100) + " hours";
    if (Math.floor(ms / 60000) > 0)
      return UTIL.formatInt(Math.round((ms / 60000) * 100) / 100) + " minutes";
    if (Math.floor(ms / 1000) > 0)
      return UTIL.formatInt(ms / 1000) + " seconds";
    return ms + " milliseconds";
  }

  // https://gist.github.com/md2perpe/8210411
  function utilPermutations(list) {
    // Empty list has one permutation
    if (list.length === 0) return [[]];
    var result = [];

    for (var i = 0; i < list.length; i++) {
      // Clone list (kind of)
      var copy = Object.create(list);
      // Cut one element from list
      var head = copy.splice(i, 1);
      // Permute rest of list
      var rest = utilPermutations(copy);
      // Add head to each permutation of rest of list
      for (var j = 0; j < rest.length; j++) {
        var next = head.concat(rest[j]);
        result.push(next);
      }
    }
    return result;
  }

  // sort by distance to center
  function utilSortByDistanceToCenter(a, b) {
    if (a.dist_c > b.dist_c) return -1;
    else if (a.dist_c < b.dist_c) return 1;
    else return 0;
  }

  // sort by the distance to a node
  function utilSortByDistanceToNode(node) {
    return function (a, b) {
      var dist_a = utilDistanceToCenter(node, a);
      var dist_b = utilDistanceToCenter(node, b);
      if (dist_a < dist_b) return -1;
      else if (dist_a > dist_b) return 1;
      else return 0;
    };
  }

  // sort by first value
  function utilSortByFirstValue(a, b) {
    if (a[0] < b[0]) return -1;
    else if (a[0] > b[0]) return 1;
    else return 0;
  }
}
