unit ConstraintParticle;

interface

uses
  System.SysUtils, System.Types, System.UITypes, System.Math,
  System.Generics.Collections;

type
  TConstraintParticle = record
    Pos: TPointF;
    Radius: Single;
    Color: TAlphaColor;
  end;

  TPointFHelper = record helper for TPointF
    procedure SetLength(aNewLength: Single);
  end;

  TConstraintResolver = class
    class function Distance(point, anchor: TPointF; distance: Double): TPointF;
  end;

  TSpatialHash = class
  private
    FCellSize: Single;
    FCells: TDictionary<Int64, TList<Integer>>;
    function GetCellKey(X, Y: Integer): Int64; inline;
  public
    constructor Create(aCellSize: Single);
    destructor Destroy; override;
    procedure Clear;
    procedure Insert(aIndex: Integer; const aPos: TPointF);
    procedure GetNearby(const aPos: TPointF; aResult: TList<Integer>);
  end;

  TConstraintParticleSimulation = class abstract
  protected
    FParticles: TArray<TConstraintParticle>;
    FMainParticle: TConstraintParticle;
    function GetParticle(Index: Integer): TConstraintParticle;
    function GetParticleCount: Integer;
  public
    destructor Destroy; override;
    procedure Update(const aMousePos: TPointF); virtual; abstract;

    property Particles[Index: Integer]: TConstraintParticle read GetParticle;
    property ParticleCount: Integer read GetParticleCount;
    property MainParticle: TConstraintParticle read FMainParticle;
    property ParticlesArray: TArray<TConstraintParticle> read FParticles;
  end;

implementation

{ TPointFHelper }

procedure TPointFHelper.SetLength(aNewLength: Single);
var
  currentLength: Single;
begin
  currentLength := Sqrt(X * X + Y * Y);
  if currentLength > 0 then
  begin
    X := X / currentLength * aNewLength;
    Y := Y / currentLength * aNewLength;
  end;
end;

{ TConstraintResolver }

class function TConstraintResolver.Distance(point, anchor: TPointF; distance: Double): TPointF;
begin
  Result := (point - anchor).Normalize * distance + anchor;
end;

{ TSpatialHash }

constructor TSpatialHash.Create(aCellSize: Single);
begin
  inherited Create;
  FCellSize := aCellSize;
  FCells := TDictionary<Int64, TList<Integer>>.Create;
end;

destructor TSpatialHash.Destroy;
var
  lst: TList<Integer>;
begin
  for lst in FCells.Values do
    lst.Free;
  FCells.Free;
  inherited;
end;

function TSpatialHash.GetCellKey(X, Y: Integer): Int64;
begin
  Result := (Int64(X) shl 32) or (Int64(Y) and $FFFFFFFF);
end;

procedure TSpatialHash.Clear;
var
  lst: TList<Integer>;
begin
  for lst in FCells.Values do
    lst.Clear;
end;

procedure TSpatialHash.Insert(aIndex: Integer; const aPos: TPointF);
var
  cellX, cellY: Integer;
  key: Int64;
  lst: TList<Integer>;
begin
  cellX := Trunc(aPos.X / FCellSize);
  cellY := Trunc(aPos.Y / FCellSize);
  key := GetCellKey(cellX, cellY);
  if not FCells.TryGetValue(key, lst) then
  begin
    lst := TList<Integer>.Create;
    FCells.Add(key, lst);
  end;
  lst.Add(aIndex);
end;

procedure TSpatialHash.GetNearby(const aPos: TPointF; aResult: TList<Integer>);
var
  cellX, cellY, dx, dy: Integer;
  key: Int64;
  lst: TList<Integer>;
begin
  cellX := Trunc(aPos.X / FCellSize);
  cellY := Trunc(aPos.Y / FCellSize);
  for dx := -1 to 1 do
    for dy := -1 to 1 do
    begin
      key := GetCellKey(cellX + dx, cellY + dy);
      if FCells.TryGetValue(key, lst) then
        aResult.AddRange(lst);
    end;
end;

{ TConstraintParticleSimulation }

destructor TConstraintParticleSimulation.Destroy;
begin
  SetLength(FParticles, 0);
  inherited;
end;

function TConstraintParticleSimulation.GetParticle(Index: Integer): TConstraintParticle;
begin
  Result := FParticles[Index];
end;

function TConstraintParticleSimulation.GetParticleCount: Integer;
begin
  Result := Length(FParticles);
end;

end.
