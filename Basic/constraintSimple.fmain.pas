unit constraintSimple.fmain;

interface

uses
  System.SysUtils, System.Types, System.UITypes, System.Classes, System.Variants,
  FMX.Types, FMX.Controls, FMX.Forms, FMX.Graphics, FMX.Dialogs, FMX.Objects,
  FMX.Controls.Presentation, FMX.StdCtrls, System.Math,
  GS.Geometry.Direction;

type
  TConstraintScene = (basicDistance,SeparateCollision,DistanceChain);

  TFMain = class(TForm)
    Selection1: TSelection;
    Rectangle1: TRectangle;
    CornerButton1: TCornerButton;
    CornerButton2: TCornerButton;
    CornerButton3: TCornerButton;
    SelectionConstraintChainSubMenu: TSelection;
    Rectangle2: TRectangle;
    CheckBoxFabrick: TCheckBox;
    TrackBar1: TTrackBar;
    cbBallCollision: TCheckBox;
    procedure FormCreate(Sender: TObject);
    procedure CornerButton2Click(Sender: TObject);

    function getRandomColor : TAlphaColor;
  private
    FCurrentScene : TConstraintScene;
    procedure SetConstaintScene(const Value: TConstraintScene);
    function GetCircles(Index: integer): TCircle;
    { Private declarations }
  public
    { Public declarations }
    procedure clearScene;
    function addCircle(aRadius : single; aPosition : TPointF; const afillColor : TAlphaColor) : TCircle;

    //Specific scene mouse move.
    procedure FormMouseMove_DistanceConstraint(Sender: TObject; Shift: TShiftState; X, Y: Single);
    procedure FormMouseMove_SeparateCollision(Sender: TObject; Shift: TShiftState; X, Y: Single);
    procedure FormMouseMove_DistanceConstraintChain(Sender: TObject; Shift: TShiftState; X, Y: Single);


    property Circles[Index : integer] : TCircle read GetCircles;

    property Scene : TConstraintScene read FCurrentScene Write SetConstaintScene;
  end;



  //A little bit better TCircle for pascal. Original JS is very readable and consise on this point.
  TCCircle = class(TCircle)
  private
    function GetPos: TPointF;
    procedure SetPos(const Value: TPointF);
    function GetRadius: single;
  published
    procedure setup(aRadius : single; aPos : TPointF; const afillColor : TAlphaColor);
    property pos : TPointF read GetPos Write SetPos;
    property radius : single read GetRadius;
  end;

  //Add powerfull "pointf.length" (vectored length form paperJs)
  TPointFTool = record helper for TPointf
    procedure setLength(aNewLength : single); //vertor grow.
  end;

  TConstraintResolver = class
    class function Distance(point, anchor: TPointF; distance: Double): TPointF;
  end;

var
  FMain: TFMain;

Const cst_SEPARATECOLL_BALL_COUNT = 40;
      cst_CHAIN_BALL_COUNT = 20;


implementation

class function TConstraintResolver.Distance(point, anchor: TPointF; distance: Double): TPointF;
begin
  Result := (point - anchor).Normalize * distance + anchor;
end;

{$R *.fmx}

function TFMain.addCircle(aRadius: single; aPosition: TPointF; const afillColor : TAlphaColor): TCircle;
begin
  result := TCircle.Create(Self);
  result.HitTest := false;
  AddObject(result);
  TCCircle(result).setup(aRadius,aPosition,afillColor);

  Selection1.BringToFront;
end;

procedure TFMain.clearScene;
begin
  for var i : integer := ChildrenCount-1 downto 0 do
    if Children.Items[i] is TCircle then begin
      RemoveObject(Children.Items[i]);
    end;
end;

procedure TFMain.CornerButton2Click(Sender: TObject);
begin
  case TCornerButton(Sender).Tag of
  10 : Scene := TConstraintScene.basicDistance;
  20 : Scene := TConstraintScene.SeparateCollision;
  30 : Scene := TConstraintScene.DistanceChain;
  end;
end;

procedure TFMain.FormCreate(Sender: TObject);
begin
  SelectionConstraintChainSubMenu.Visible := false;
  CornerButton1.OnClick(CornerButton1);
end;

procedure TFMain.FormMouseMove_DistanceConstraintChain(Sender: TObject;
  Shift: TShiftState; X, Y: Single);
var i : integer;
    balls : TArray<TCCircle>;
    mousepos : TPointF;
begin
  setlength(balls,cst_CHAIN_BALL_COUNT);
  for i := 0 to cst_CHAIN_BALL_COUNT-1 do
    balls[i] := TCCircle(Circles[i]);

  mousepos := pointf(X,Y);
  balls[0].pos := mousepos;
  for i := 1 to length(balls)-1 do
    balls[i].pos := TConstraintResolver.Distance(balls[i].pos,balls[i-1].pos,TrackBar1.Value);

  //https://zalo.github.io/blog/constraints/#fabrik-chain
  if CheckBoxFabrick.IsChecked then begin
    balls[length(balls)-1].pos := pointf(ClientWidth/2,ClientHeight/2);
    for i := length(balls)-1 downto 1 do
      balls[i-1].pos := TConstraintResolver.Distance(balls[i-1].pos,balls[i].pos,TrackBar1.Value);
  end;

  //Perform ball collision.
  if cbBallCollision.IsChecked then begin
    //separate balls
    var j : integer;
    var ToNext,lOffset : TPointf;
    var lradius : double;
    for i := 0 to length(balls)-1 do
      for j := i to length(balls)-1 do begin
        if balls[i] = balls[j] then
          continue;

        toNext := balls[j].pos - balls[i].pos;
        lradius := balls[j].radius + balls[i].radius;
        if toNext.Length <= lradius then begin
          toNext.setLength(lradius);
          loffset := balls[j].pos - balls[i].pos - toNext;
          loffset := loffset/2;
          balls[i].pos := balls[i].pos + loffset;
          balls[j].pos := balls[j].pos - loffset;
        end;
      end;
  end;
end;

procedure verletIntegrate(var curPt, prevPt : TPointf);
var ltemp : TpointF;
begin
  ltemp := curPt;
  curPt := (curPt + (curPt - prevPt));
  prevPt := ltemp;
end;


procedure TFMain.FormMouseMove_DistanceConstraint(Sender: TObject;
  Shift: TShiftState; X, Y: Single);
var toNext : TPointf;
    mousecoord : TPointf;
    circle, ball : TCCircle;
begin
  circle := TCCircle(Circles[0]);
  ball := TCCircle(Circles[1]);

  mousecoord := Pointf(x,y);

  Circle.pos := mousecoord;

  toNext := mousecoord - ball.pos;
  if toNext.Length>circle.radius-ball.radius then
    ball.pos := TConstraintResolver.Distance(ball.pos,mousecoord,circle.radius-ball.radius);
end;

procedure TFMain.FormMouseMove_SeparateCollision(Sender: TObject;
  Shift: TShiftState; X, Y: Single);
var toNext : TPointf;
    mousecoord : TPointf;
    circle,b : TCCircle;
    balls : TArray<TCCircle>;
    i,j : integer;
    lradius : single;
    loffset : TPointF;
    lDir : TDirectionalObject;
begin
  circle := TCCircle(Circles[0]);
  setLength(balls,cst_SEPARATECOLL_BALL_COUNT);
  for i := 1 to cst_SEPARATECOLL_BALL_COUNT do
    balls[i-1] := TCCircle(Circles[i]);

  mousecoord := Pointf(x,y);

  Circle.pos := mousecoord;

  //Constraint for main circle.
  for b in balls do begin
    toNext := Circle.pos - b.pos;
    if toNext.Length<circle.radius+b.radius then
      toNext.setLength(circle.radius+b.radius);
      loffset := circle.pos - b.pos - toNext;
      b.pos := b.pos + loffset;
  end;

  //separate balls
  for i := 0 to length(balls)-1 do
    for j := i to length(balls)-1 do begin
      if balls[i] = balls[j] then
        continue;

      toNext := balls[j].pos - balls[i].pos;
      lradius := balls[j].radius + balls[i].radius;
      if toNext.Length <= lradius then begin
        toNext.setLength(lradius);
        loffset := balls[j].pos - balls[i].pos - toNext;
        loffset := loffset/2;
        balls[i].pos := balls[i].pos + loffset;
        balls[j].pos := balls[j].pos - loffset;
      end;
    end;
end;

function TFMain.GetCircles(Index: integer): TCircle;
var i,c : integer;
    l : TArray<TFmxObject>;
begin
  l := Children.ToArray;
  c := 0;
  for I := Low(l) to High(l) do
    if l[i] is TCircle then begin
      if c=index then begin
        result := TCircle(l[i]);
        break;
      end;
      inc(c);
    end;
end;

function TFMain.getRandomColor: TAlphaColor;
begin
  result := TAlphaColorF.Create(Random(255)/255,Random(255)/255,Random(255)/255,1).ToAlphaColor
end;

procedure TFMain.SetConstaintScene(const Value: TConstraintScene);
var i,xj,yj : integer;
begin
  CornerButton1.IsPressed := false;
  CornerButton2.IsPressed := false;
  CornerButton3.IsPressed := false;
  SelectionConstraintChainSubMenu.Visible := False;
  OnMouseMove := nil;

  clearScene;
  case value  of

    basicDistance: begin
      CornerButton1.IsPressed := true;
      addCircle(50,pointF(400,400),TAlphaColors.White);
      addCircle(15,pointF(400,400),TAlphaColors.Black);
      OnMouseMove := FormMouseMove_DistanceConstraint;
    end;

    SeparateCollision: begin
      CornerButton2.IsPressed := true;
      addCircle(50,pointF(400,400),TAlphaColors.White);
      xj := 0;
      yj := xj;
      for i := 1 to cst_SEPARATECOLL_BALL_COUNT do begin
        if i Mod 10 = 0 then begin
          xj := 0;
          inc(yj);
        end;
        addCircle(15,pointF(400+(xj*20),400+(yj*20)),getRandomColor);
        inc(xj);
      end;
      OnMouseMove := FormMouseMove_SeparateCollision;
    end;

    DistanceChain: begin
      CornerButton3.IsPressed := true;
      SelectionConstraintChainSubMenu.Visible := true;
      for i := 1 to cst_CHAIN_BALL_COUNT do begin
        addCircle(15,pointF(400+(i*50),400),getRandomColor);
        inc(xj);
      end;
      OnMouseMove := FormMouseMove_DistanceConstraintChain;
    end;
  end;
end;

{ TCCircle }

function TCCircle.GetPos: TPointF;
begin
  result := Position.Point + PointF(radius,radius);
end;

function TCCircle.GetRadius: single;
begin
  result := Width/2;
end;

procedure TCCircle.SetPos(const Value: TPointF);
begin
  Position.Point := value - pointf(radius,radius);
end;


procedure TCCircle.setup(aRadius: single; aPos: TPointF; const afillColor : TAlphaColor);
begin
  Width := aRadius*2;
  Height := Width;
  Fill.Color := aFillColor;
  pos := aPos;
end;

{ TPointFTool }

procedure TPointFTool.setLength(aNewLength: single);
var l  : TDirectionalObject;
    la : TPt;
begin
  la := point(X,Y);
  l := TDirectionalObject.Create(0,0,1);
  try
    l.LookAt(la);
    l.Norm := aNewLength;
    la := l.GetPointedCoord;
    X := la.X;
    Y := la.Y;
  finally
    FreeAndNil(l);
  end;
end;

end.
