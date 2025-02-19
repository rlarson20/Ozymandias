graph TD;
  Organization["Organization\n- Mission\n- Requirements\n- Objectives\n- Goals\n- Scenarios"];
  Perspective["Perspective\n- Contains Objects\n- Defines\n- Exposes Concerns"];
  Function["Function\n- Logical structure\n- Behavior\n- Interfaces\n- Constraints"];
  Information["Information\n- Data\n- Metadata\n- Rules"];
  Component["Component\n- Type\n- Attributes\n- Ports\n- Location"];
  Environment["Environment\n- Physical Environs\n- Attributes"];
  Communication["Communication\n- Protocol stack\n- Standards"];
  Connector["Connector\n- Type\n- Attributes"];

  Organization -->|ComposedOf| Organization;
  Organization -->|Owns/Operates| Perspective;
  Organization -->|FulfilledBy| Function;

  Function -->|ComposedOf| Function;
  Function -->|Fulfills| Organization;
  Function -->|Calls| Function;
  Function -->|Produces| Information;
  Function -->|Consumes| Information;
  Function -->|IsAllocatedTo| Component;
  Function -->|ContainsInstances| Component;
  Function -->|ProvidesService| Communication;
  
  Component -->|ComposedOf| Component;
  Component -->|ImplementedOn| Communication;
  Component -->|AssociatedWith| Connector;
  Component -->|Affects| Environment;
  
  Connector -->|ConnectVia| Component;
  Connector -->|ConnectToPort| Component;
  
  Communication -->|Uses| Function;
